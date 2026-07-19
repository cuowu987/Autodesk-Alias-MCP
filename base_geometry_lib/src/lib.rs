mod point;
pub use point::*;
mod atomic;
pub use atomic::*;



use std::panic;

/// #捕获 panic
pub fn safe_run<F, T, F1>(f: F, f1: F1) -> Option<T>
where
    F: FnOnce() -> T + panic::UnwindSafe,
    F1: FnOnce(&str),
{
    match panic::catch_unwind(f) {
        Ok(v) => Some(v),
        Err(err) => {
            let msg = if let Some(s) = err.downcast_ref::<&str>() {
                *s
            } else if let Some(s) = err.downcast_ref::<String>() {
                s.as_str()
            } else {
                "unknown panic"
            };
            f1(msg);
            None
        }
    }
}

/// # 捕获 String
pub fn safe_run2<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce() -> Result<T, String>,
{
    f()
}

use std::thread;

pub trait Parallel<T> {
    fn parallel<R, F>(&self, func: F) -> Vec<R>
    where
        R: Send,
        F: Fn(&T) -> R + Sync + Send;
}

impl<T: Sync> Parallel<T> for [T] {
    fn parallel<R, F>(&self, func: F) -> Vec<R>
    where
        R: Send,
        F: Fn(&T) -> R + Sync + Send,
    {
        let cpunum = 20.min(self.len());
        thread::scope(|s| {
            let mut handles = Vec::new();
            let data = self; // 引用 Vec
            let func = &func;

            for thread_id in 0..cpunum {
                let handle = s.spawn(move || {
                    let mut local_results = Vec::new();
                    for i in (thread_id..data.len()).step_by(cpunum) {
                        let result = func(&data[i]);
                        local_results.push((i, result)); // 保存索引
                    }
                    local_results
                });
                handles.push(handle);
            }

            // join + 合并
            let mut results: Vec<(usize, R)> = handles
                .into_iter()
                .flat_map(|h| h.join().unwrap())
                .collect();

            // 按原 Vec 顺序排序
            results.sort_by_key(|(i, _)| *i);

            // 去掉索引，保留结果
            results.into_iter().map(|(_, r)| r).collect()
        })
    }
}

pub fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![start];
    }
    let step = (end - start) / (n as f64 - 1.0);
    (0..n).map(|i| start + i as f64 * step).collect()
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return vec![];
    }
    let row_len = v[0].len();
    assert!(
        v.iter().all(|row| row.len() == row_len),
        "不规则矩阵，无法转置"
    );

    let mut iters: Vec<_> = v.into_iter().map(|row| row.into_iter()).collect();
    let mut result = Vec::with_capacity(row_len);

    for _ in 0..row_len {
        let mut new_row = Vec::with_capacity(iters.len());
        for iter in iters.iter_mut() {
            new_row.push(iter.next().unwrap());
        }
        result.push(new_row);
    }
    result
}
