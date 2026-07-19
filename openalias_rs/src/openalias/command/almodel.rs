#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_camel_case_types)]

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum curveFormType {
    kClosed = 0,
    kOpen = 1,
    kPeriodic = 2,
    kInvalidCurve = 3,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlClusterRestrict {
    kMultiCluster = 0,
    kExclusiveCluster = 1,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlOutputType {
    kStdout = 0,
    kStderr = 1,
    kPrompt = 2,
    kErrlog = 3,
    kPromptNoHistory = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlOutputTypeMask {
    oStdout = 1u32 << 0,
    oStderr = 1u32 << 1,
    oPrompt = 1u32 << 2,
    oErrlog = 1u32 << 3,
    oPromptNoHistory = 1u32 << 4,
}
