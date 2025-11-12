//! 🐍 PyO3 Python集成绑定
//!
//! 为Pixly v3.0性能核心提供Python原生接口：
//! - PyO3零拷贝数据交换
//! - NumPy数组直接支持
//! - 异步任务支持
//! - Python GIL优化
//!
//! Python调用示例:
//! ```python
//! import pixly_performance_core
//! 
//! # 初始化性能核心
//! core = pixly_performance_core.PerformanceCore()
//! 
//! # 处理图像
//! result = core.process_image(image_bytes, "resize", width=800, height=600)
//! ```

use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use pyo3::wrap_pyfunction;
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use anyhow::Result;

use super::{
    PerformanceCore, PerformanceConfig, ImageOperation, 
    BenchmarkCase, BenchmarkResult, SystemInfo
};

/// Python性能核心包装器
#[pyclass]
pub struct PyPerformanceCore {
    inner: PerformanceCore,
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl PyPerformanceCore {
    /// 创建新的性能核心实例
    #[new]
    #[pyo3(signature = (
        enable_simd=true,
        enable_gpu=true, 
        gpu_device_id=0,
        worker_threads=0,
        memory_pool_size_mb=1024,
        enable_profiling=true
    ))]
    fn new(
        enable_simd: bool,
        enable_gpu: bool,
        gpu_device_id: u32,
        worker_threads: usize,
        memory_pool_size_mb: usize,
        enable_profiling: bool,
    ) -> PyResult<Self> {
        
        let config = PerformanceConfig {
            enable_simd,
            enable_gpu,
            gpu_device_id,
            worker_threads,
            memory_pool_size_mb,
            enable_profiling,
        };
        
        // 创建Tokio运行时
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("创建异步运行时失败: {}", e)
            ))?;
        
        // 在运行时中初始化性能核心
        let inner = runtime.block_on(async {
            PerformanceCore::new(config).await
        }).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("初始化性能核心失败: {}", e)
        ))?;
        
        Ok(Self { inner, runtime })
    }
    
    /// 处理图像
    #[pyo3(signature = (image_data, operation, **kwargs))]
    fn process_image(
        &self,
        py: Python,
        image_data: &PyBytes,
        operation: &str,
        kwargs: Option<&PyDict>,
    ) -> PyResult<PyObject> {
        
        // 解析操作参数
        let image_op = parse_image_operation(operation, kwargs)?;
        
        // 获取图像数据
        let data = image_data.as_bytes();
        
        // 释放GIL并执行异步处理
        py.allow_threads(|| {
            self.runtime.block_on(async {
                self.inner.process_image(data, image_op).await
            })
        }).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("图像处理失败: {}", e)
        ))?;
        
        // 返回处理后的数据
        let result_bytes = py.allow_threads(|| {
            self.runtime.block_on(async {
                self.inner.process_image(data, image_op).await
            })
        }).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("图像处理失败: {}", e)
        ))?;
        
        Ok(PyBytes::new(py, &result_bytes).into())
    }
    
    /// 批量处理图像
    fn process_images_batch(
        &self,
        py: Python,
        images: Vec<&PyBytes>,
        operation: &str,
        kwargs: Option<&PyDict>,
    ) -> PyResult<Vec<PyObject>> {
        
        let image_op = parse_image_operation(operation, kwargs)?;
        let mut results = Vec::new();
        
        for image_data in images {
            let data = image_data.as_bytes();
            
            let result_bytes = py.allow_threads(|| {
                self.runtime.block_on(async {
                    self.inner.process_image(data, image_op.clone()).await
                })
            }).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("批量处理失败: {}", e)
            ))?;
            
            results.push(PyBytes::new(py, &result_bytes).into());
        }
        
        Ok(results)
    }
    
    /// 处理NumPy数组
    fn process_numpy_array(
        &self,
        py: Python,
        array: PyReadonlyArray1<u8>,
        operation: &str,
        kwargs: Option<&PyDict>,
    ) -> PyResult<PyObject> {
        
        let image_op = parse_image_operation(operation, kwargs)?;
        let data = array.as_slice()?;
        
        let result_bytes = py.allow_threads(|| {
            self.runtime.block_on(async {
                self.inner.process_image(data, image_op).await
            })
        }).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            format!("NumPy数组处理失败: {}", e)
        ))?;
        
        // 返回NumPy数组
        let result_array = PyArray1::from_slice(py, &result_bytes);
        Ok(result_array.into())
    }
    
    /// 获取性能统计
    fn get_stats(&self, py: Python) -> PyResult<PyObject> {
        let stats = self.inner.get_stats();
        
        let stats_dict = py.dict();
        stats_dict.set_item("total_tasks", stats.total_tasks)?;
        stats_dict.set_item("successful_tasks", stats.successful_tasks)?;
        stats_dict.set_item("total_processing_time_ns", stats.total_processing_time_ns)?;
        stats_dict.set_item("avg_latency_ns", stats.avg_latency_ns)?;
        stats_dict.set_item("peak_throughput", stats.peak_throughput)?;
        stats_dict.set_item("simd_usage_count", stats.simd_usage_count)?;
        stats_dict.set_item("gpu_usage_count", stats.gpu_usage_count)?;
        stats_dict.set_item("memory_allocations", stats.memory_allocations)?;
        
        Ok(stats_dict.into())
    }
    
    /// 获取系统信息
    fn get_system_info(&self, py: Python) -> PyResult<PyObject> {
        let system_info = self.inner.get_system_info();
        
        let info_dict = py.dict();
        
        // CPU信息
        let cpu_dict = py.dict();
        cpu_dict.set_item("cores", system_info.cpu_info.cores)?;
        cpu_dict.set_item("logical_cores", system_info.cpu_info.logical_cores)?;
        cpu_dict.set_item("supports_avx2", system_info.cpu_info.supports_avx2)?;
        cpu_dict.set_item("supports_avx512", system_info.cpu_info.supports_avx512)?;
        cpu_dict.set_item("supports_neon", system_info.cpu_info.supports_neon)?;
        cpu_dict.set_item("cache_line_size", system_info.cpu_info.cache_line_size)?;
        info_dict.set_item("cpu_info", cpu_dict)?;
        
        // GPU信息
        if let Some(ref gpu_info) = system_info.gpu_info {
            let gpu_dict = py.dict();
            gpu_dict.set_item("name", &gpu_info.name)?;
            gpu_dict.set_item("vendor", &gpu_info.vendor)?;
            gpu_dict.set_item("memory_mb", gpu_info.memory_mb)?;
            gpu_dict.set_item("compute_units", gpu_info.compute_units)?;
            gpu_dict.set_item("supports_compute", gpu_info.supports_compute)?;
            info_dict.set_item("gpu_info", gpu_dict)?;
        } else {
            info_dict.set_item("gpu_info", py.None())?;
        }
        
        // 配置信息
        let config_dict = py.dict();
        config_dict.set_item("enable_simd", system_info.config.enable_simd)?;
        config_dict.set_item("enable_gpu", system_info.config.enable_gpu)?;
        config_dict.set_item("gpu_device_id", system_info.config.gpu_device_id)?;
        config_dict.set_item("worker_threads", system_info.config.worker_threads)?;
        config_dict.set_item("memory_pool_size_mb", system_info.config.memory_pool_size_mb)?;
        config_dict.set_item("enable_profiling", system_info.config.enable_profiling)?;
        info_dict.set_item("config", config_dict)?;
        
        info_dict.set_item("uptime_seconds", system_info.uptime_seconds)?;
        
        Ok(info_dict.into())
    }
    
    /// 运行性能基准测试
    #[pyo3(signature = (test_cases))]
    fn run_benchmark(
        &self,
        py: Python,
        test_cases: Vec<PyBenchmarkCase>,
    ) -> PyResult<Vec<PyObject>> {
        
        // 转换测试用例
        let rust_test_cases: Result<Vec<BenchmarkCase>, _> = test_cases
            .into_iter()
            .map(|case| case.to_rust())
            .collect();
        
        let rust_test_cases = rust_test_cases
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("测试用例转换失败: {}", e)
            ))?;
        
        // 执行基准测试
        let results = py.allow_threads(|| {
            self.runtime.block_on(async {
                self.inner.run_benchmark(rust_test_cases).await
            })
        });
        
        // 转换结果
        let mut py_results = Vec::new();
        for result in results {
            let result_dict = py.dict();
            result_dict.set_item("name", result.name)?;
            result_dict.set_item("iterations", result.iterations)?;
            result_dict.set_item("success_count", result.success_count)?;
            result_dict.set_item("total_duration_ms", result.total_duration.as_millis() as u64)?;
            result_dict.set_item("avg_latency_ms", result.avg_latency.as_millis() as u64)?;
            result_dict.set_item("throughput", result.throughput)?;
            result_dict.set_item("data_processed_mb", result.data_processed_mb)?;
            
            py_results.push(result_dict.into());
        }
        
        Ok(py_results)
    }
    
    /// 预热性能核心 (预分配资源)
    fn warmup(&self, py: Python) -> PyResult<()> {
        // 创建小测试数据进行预热
        let test_data = vec![0u8; 1024]; // 1KB测试数据
        
        py.allow_threads(|| {
            self.runtime.block_on(async {
                let _ = self.inner.process_image(&test_data, ImageOperation::Enhance).await;
            })
        });
        
        Ok(())
    }
}

/// Python基准测试用例
#[pyclass]
#[derive(Clone)]
pub struct PyBenchmarkCase {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    test_data: Vec<u8>,
    #[pyo3(get, set)]
    operation: String,
    #[pyo3(get, set)]
    iterations: usize,
    #[pyo3(get, set)]
    operation_params: Option<HashMap<String, String>>,
}

#[pymethods]
impl PyBenchmarkCase {
    #[new]
    fn new(
        name: String,
        test_data: Vec<u8>,
        operation: String,
        iterations: usize,
        operation_params: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            name,
            test_data,
            operation,
            iterations,
            operation_params,
        }
    }
}

impl PyBenchmarkCase {
    fn to_rust(&self) -> Result<BenchmarkCase> {
        let operation = match self.operation.as_str() {
            "resize" => {
                let params = self.operation_params.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("resize操作需要width和height参数"))?;
                
                let width: u32 = params.get("width")
                    .ok_or_else(|| anyhow::anyhow!("缺少width参数"))?
                    .parse()?;
                
                let height: u32 = params.get("height")
                    .ok_or_else(|| anyhow::anyhow!("缺少height参数"))?
                    .parse()?;
                
                ImageOperation::Resize { width, height }
            }
            "compress" => {
                let quality = if let Some(params) = &self.operation_params {
                    params.get("quality")
                        .map(|q| q.parse().unwrap_or(80))
                        .unwrap_or(80)
                } else {
                    80
                };
                
                ImageOperation::Compress { quality: quality as u8 }
            }
            "enhance" => ImageOperation::Enhance,
            _ => anyhow::bail!("不支持的操作: {}", self.operation),
        };
        
        Ok(BenchmarkCase {
            name: self.name.clone(),
            test_data: self.test_data.clone(),
            operation,
            iterations: self.iterations,
        })
    }
}

/// 解析图像操作
fn parse_image_operation(operation: &str, kwargs: Option<&PyDict>) -> PyResult<ImageOperation> {
    match operation {
        "resize" => {
            let kwargs = kwargs.ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>("resize操作需要width和height参数")
            })?;
            
            let width: u32 = kwargs
                .get_item("width")?
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("缺少width参数"))?
                .extract()?;
            
            let height: u32 = kwargs
                .get_item("height")?
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("缺少height参数"))?
                .extract()?;
            
            Ok(ImageOperation::Resize { width, height })
        }
        "compress" => {
            let quality = if let Some(kwargs) = kwargs {
                kwargs
                    .get_item("quality")?
                    .map(|q| q.extract().unwrap_or(80))
                    .unwrap_or(80)
            } else {
                80
            };
            
            Ok(ImageOperation::Compress { quality: quality as u8 })
        }
        "enhance" => Ok(ImageOperation::Enhance),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("不支持的操作: {}", operation)
        )),
    }
}

/// 便利函数：创建性能核心
#[pyfunction]
#[pyo3(signature = (
    enable_simd=true,
    enable_gpu=true,
    gpu_device_id=0,
    worker_threads=0,
    memory_pool_size_mb=1024,
    enable_profiling=true
))]
fn create_performance_core(
    enable_simd: bool,
    enable_gpu: bool,
    gpu_device_id: u32,
    worker_threads: usize,
    memory_pool_size_mb: usize,
    enable_profiling: bool,
) -> PyResult<PyPerformanceCore> {
    PyPerformanceCore::new(
        enable_simd,
        enable_gpu,
        gpu_device_id,
        worker_threads,
        memory_pool_size_mb,
        enable_profiling,
    )
}

/// 便利函数：检测系统SIMD能力
#[pyfunction]
fn detect_simd_capabilities(py: Python) -> PyResult<PyObject> {
    let caps_dict = py.dict();
    
    #[cfg(target_arch = "x86_64")]
    {
        caps_dict.set_item("sse2", is_x86_feature_detected!("sse2"))?;
        caps_dict.set_item("sse4_1", is_x86_feature_detected!("sse4.1"))?;
        caps_dict.set_item("avx", is_x86_feature_detected!("avx"))?;
        caps_dict.set_item("avx2", is_x86_feature_detected!("avx2"))?;
        caps_dict.set_item("avx512f", is_x86_feature_detected!("avx512f"))?;
        caps_dict.set_item("fma", is_x86_feature_detected!("fma"))?;
        caps_dict.set_item("neon", false)?;
    }
    
    #[cfg(target_arch = "aarch64")]
    {
        caps_dict.set_item("sse2", false)?;
        caps_dict.set_item("sse4_1", false)?;
        caps_dict.set_item("avx", false)?;
        caps_dict.set_item("avx2", false)?;
        caps_dict.set_item("avx512f", false)?;
        caps_dict.set_item("fma", false)?;
        caps_dict.set_item("neon", true)?;
    }
    
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        caps_dict.set_item("sse2", false)?;
        caps_dict.set_item("sse4_1", false)?;
        caps_dict.set_item("avx", false)?;
        caps_dict.set_item("avx2", false)?;
        caps_dict.set_item("avx512f", false)?;
        caps_dict.set_item("fma", false)?;
        caps_dict.set_item("neon", false)?;
    }
    
    Ok(caps_dict.into())
}

/// Python模块定义
#[pymodule]
fn pixly_performance_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPerformanceCore>()?;
    m.add_class::<PyBenchmarkCase>()?;
    m.add_function(wrap_pyfunction!(create_performance_core, m)?)?;
    m.add_function(wrap_pyfunction!(detect_simd_capabilities, m)?)?;
    
    // 模块信息
    m.add("__version__", "3.0.0")?;
    m.add("__author__", "PIXLY Team")?;
    m.add("__description__", "Ultra-high performance Rust core for PIXLY v3.0")?;
    
    Ok(())
}
