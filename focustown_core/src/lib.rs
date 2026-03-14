// focustown_core/src/lib.rs
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Bound;

/// 尺寸
#[pyclass]
#[derive(Clone, Copy, Debug)]
pub struct Size {
    #[pyo3(get, set)]
    pub width: i32,
    #[pyo3(get, set)]
    pub height: i32,
}

#[pymethods]
impl Size {
    #[new]
    fn new(width: i32, height: i32) -> Self {
        Size { width, height }
    }
    
    fn __repr__(&self) -> String {
        format!("Size({}, {})", self.width, self.height)
    }
}

/// 比例计算
#[pyclass]
pub struct AspectRatioConstraint {
    ratio: f64,
    threshold: i32,  // 防抖阈值
}

#[pymethods]
impl AspectRatioConstraint {
    #[new]
    fn new(ratio_width: i32, ratio_height: i32) -> Self {
        AspectRatioConstraint {
            ratio: ratio_width as f64 / ratio_height as f64,
            threshold: 2,
        }
    }
    
    /// 计算约束后
    fn calculate(&self, current: &Size, old: &Size) -> Size {
        // 如果变化小于阈值，保持原样
        let dw = (current.width - old.width).abs();
        let dh = (current.height - old.height).abs();
        
        if dw < self.threshold && dh < self.threshold {
            return *old;
        }
        
        // 判断拖动方向
        if dw > dh {
            // 宽度主，高度跟
            let new_height = (current.width as f64 / self.ratio) as i32;
            Size::new(current.width, new_height)
        } else {
            // 高度主，宽度跟
            let new_width = (current.height as f64 * self.ratio) as i32;
            Size::new(new_width, current.height)
        }
    }
    
    /// 初始化
    fn fit_size(&self, width: i32, height: i32) -> Size {
        let current_ratio = width as f64 / height as f64;
        
        if current_ratio > self.ratio {
            // 宽了，以高度为准
            Size::new((height as f64 * self.ratio) as i32, height)
        } else {
            // 高了，以宽度为准
            Size::new(width, (width as f64 / self.ratio) as i32)
        }
    }
    
    /// 防抖阈值
    fn set_threshold(&mut self, px: i32) {
        self.threshold = px;
    }
    
    #[getter]
    fn ratio(&self) -> f64 {
        self.ratio
    }
}

/// 颜色主题工具
#[pyfunction]
fn interpolate_color(color1: &str, color2: &str, factor: f64) -> PyResult<String> {
    fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some((r, g, b))
    }
    
    let (r1, g1, b1) = hex_to_rgb(color1).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err("Invalid color1 format")
    })?;
    let (r2, g2, b2) = hex_to_rgb(color2).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err("Invalid color2 format")
    })?;
    
    let r = (r1 as f64 + (r2 as f64 - r1 as f64) * factor) as u8;
    let g = (g1 as f64 + (g2 as f64 - g1 as f64) * factor) as u8;
    let b = (b1 as f64 + (b2 as f64 - b1 as f64) * factor) as u8;
    
    Ok(format!("#{:02x}{:02x}{:02x}", r, g, b))
}

#[pymodule]
fn focustown_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Size>()?;
    m.add_class::<AspectRatioConstraint>()?;
    m.add_function(wrap_pyfunction!(interpolate_color, m)?)?;
    Ok(())
}