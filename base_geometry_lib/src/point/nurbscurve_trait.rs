#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::*;
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum curve_form_type {
    kClosed,
    kOpen,
    kPeriodic,
    kInvalidCurve,
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum mirror_type {
    none,
    WorldXY,
    WorldYZ,
    WorldZX,
}

fn CurveParamsByKnots(cv_count: usize, knots: Vec<f64>) -> Vec<f64> {
    // 特殊处理：knots 上的参数必须保留（曲线可能有间断点）
    let mut params = Vec::new();

    // 去重
    let knot_set: Vec<f64> = {
        let mut tmp = knots.clone();
        tmp.sort_by(|a, b| a.partial_cmp(b).unwrap());
        tmp.dedup();
        tmp
    };

    if knot_set.len() < 2 {
        return knot_set; // 无法插值
    }

    let num = cv_count.saturating_sub(knot_set.len());

    for i in 0..knot_set.len() - 1 {
        let a = knot_set[i];
        let b = knot_set[i + 1];
        let domain = RU_Interval::createByStartAndEnd(a, b);

        // 理应 num / (knot_set.Count() - 1) + 1, 但为了提升 fit 效果，加 2
        let div = num / (knot_set.len() - 1) + 2;
        let range = domain.range(div);

        // 去掉最后一个，避免重复
        for j in 0..range.len() - 1 {
            params.push(range[j]);
        }
    }

    // 尾补充
    if let Some(&last) = knot_set.last() {
        params.push(last);
    }
    params
}

fn bezier_extend_end(cvs: &[RU_3dPoint], end_ratio: f64) -> Vec<RU_3dPoint> {
    let mut cvs_1 = cvs.to_vec();
    let mut cvs_new = Vec::new();

    // cvs_new.push_back(cvs[0]);
    cvs_new.push(cvs_1[0]);

    while cvs_1.len() != 1 {
        let mut cvs_2 = Vec::with_capacity(cvs_1.len() - 1);

        for i in 0..cvs_1.len() - 1 {
            let p = cvs_1[i] + (cvs_1[i + 1] - cvs_1[i]) * (1.0 + end_ratio);
            cvs_2.push(p);
        }

        // cvs_new.push_back(cvs_2[0]);
        cvs_new.push(cvs_2[0]);

        cvs_1 = cvs_2;
    }

    cvs_new
}

pub trait NurbsCurve_Base_Trait {
    fn Knots(&self) -> Vec<f64>;
    fn Degree_NurbsCurveTraitbase(&self) -> usize;
    fn CVs(&self) -> Vec<RU_3dPoint>;
    fn Weights(&self) -> Vec<f64>;
    fn Form(&self) -> curve_form_type;
    ////////////////////////////////////////////////////////////////////////

    fn cvCount(&self) -> usize {
        self.CVs().len()
    }
    fn ParamsByKnots(&self) -> Vec<f64> {
        let cv_count = self.CVs().len();
        let knots = self.Knots();
        CurveParamsByKnots(cv_count, knots)
    }
    fn BoundingBox_NurbsCurveBaseTrait(&self) -> RU_BoundingBox {
        let mut bbox = RU_BoundingBox::default();
        for cv in self.CVs() {
            bbox += cv;
        }
        bbox
    }

    fn CVs_Center(&self) -> RU_3dPoint {
        let cvs = self.CVs();
        let mut center = RU_3dPoint::new(0.0, 0.0, 0.0);
        for cv in &cvs {
            center = center + *cv;
        }
        center = center / cvs.len() as f64;
        center
    }

    fn AliasKnots(&self) -> Vec<f64> {
        let knots = self.Knots();
        let degree = self.Degree_NurbsCurveTraitbase();
        let aliasKnotsNum = knots.len() - 2 * degree + 2;
        let mut aliasKnots = Vec::with_capacity(aliasKnotsNum);
        for i in 0..aliasKnotsNum {
            aliasKnots.push(knots[degree - 1 + i]);
        }
        aliasKnots
    }

    fn AliasCVs(&self) -> Vec<RU_4dPoint> {
        let cvs = self.CVs();
        let weights = self.Weights();
        let mut aliasCVs = Vec::with_capacity(cvs.len());
        for i in 0..cvs.len() {
            aliasCVs.push(RU_4dPoint::new(cvs[i].x, cvs[i].y, cvs[i].z, weights[i]));
        }
        aliasCVs
    }
    fn CurvoKnots(&self) -> Vec<f64> {
        let mut knots = self.Knots();
        knots.insert(0, knots[0]);
        knots.push(knots[knots.len() - 1]);
        knots
    }
    fn Dimension_1(&self) -> usize {
        match self.CVs().iter().all(|p| p.z == 0.0) {
            true => 2,
            false => 3,
        }
    }

}
pub trait NurbsCurve_Trait: NurbsCurve_Base_Trait {
    fn SetCVs(&mut self, cvs: &Vec<RU_3dPoint>) -> Result<(), String>;

    fn bezier_extend(&mut self, start_ratio: f64, end_ratio: f64) -> Result<(), String> {
        let mut cvs = self.CVs();
        let degree = self.Degree_NurbsCurveTraitbase();
        if cvs.len() != degree + 1 {
            return Err("cvs size not equal to degree + 1".into());
        }
        // extend end
        cvs = bezier_extend_end(&cvs, end_ratio);
        // reverse
        cvs.reverse();
        // extend start (注意比例修正)
        cvs = bezier_extend_end(&cvs, start_ratio / (1.0 + end_ratio));
        // reverse back
        cvs.reverse();
        self.SetCVs(&cvs)?;
        Ok(())
    }
    /// # nurbs曲线镜像
    ///alcurve必须实体化后才可以镜像
    fn mirror(&mut self, mirrorType: &mirror_type) -> Result<(), String> {
        let cvs = self
            .CVs()
            .iter()
            .map(|cv| match mirrorType {
                mirror_type::WorldXY => RU_3dPoint::new(cv.x, cv.y, -cv.z),
                mirror_type::WorldZX => RU_3dPoint::new(cv.x, -cv.y, cv.z),
                mirror_type::WorldYZ => RU_3dPoint::new(-cv.x, cv.y, cv.z),
                mirror_type::none => *cv,
            })
            .collect::<Vec<_>>();
        self.SetCVs(&cvs)?;
        Ok(())
    }
    
}
