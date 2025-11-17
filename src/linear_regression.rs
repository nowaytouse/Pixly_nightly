// 📊 多元线性回归 - 从Go ML代码提取
// 用于预测: Quality = β0 + β1*FileSize + β2*Resolution + β3*Complexity + ...
//
// 核心算法 (从Go代码提取):
// - 最小二乘法 (Ordinary Least Squares)
// - 岭回归 (Ridge Regression) - 处理多重共线性
// - R²决定系数计算
// - 均方误差 (MSE) 计算

use std::collections::HashMap;

/// 多元线性回归模型 (从Go的MultipleLinearRegression提取)
pub struct MultipleLinearRegression {
    coefficients: Vec<f64>,  // 回归系数 [β0, β1, β2, ...]
    feature_names: Vec<String>, // 特征名称
    trained: bool,
    
    // 统计指标
    r_squared: f64,  // R²决定系数
    mse: f64,        // 均方误差
    sample_size: usize,
}

impl MultipleLinearRegression {
    /// 创建新的多元线性回归模型
    pub fn new() -> Self {
        Self {
            coefficients: Vec::new(),
            feature_names: vec![
                "intercept".to_string(),         // β0 截距
                "file_size_mb".to_string(),      // β1 文件大小
                "megapixels".to_string(),        // β2 百万像素
                "complexity".to_string(),        // β3 复杂度
                "has_alpha".to_string(),         // β4 透明度
                "is_animated".to_string(),       // β5 动画
                "estimated_quality".to_string(), // β6 当前质量
                "noise_level".to_string(),       // β7 噪声水平
            ],
            trained: false,
            r_squared: 0.0,
            mse: 0.0,
            sample_size: 0,
        }
    }
    
    /// 训练模型 - 最小二乘法 (从Go代码提取)
    /// 
    /// # 参数
    /// * `x` - 特征矩阵 \[n_samples, n_features\]
    /// * `y` - 目标值 \[n_samples\] (如quality)
    pub fn train(&mut self, x: &[Vec<f64>], y: &[f64]) -> Result<(), String> {
        if x.is_empty() || y.is_empty() {
            return Err("训练数据为空".to_string());
        }
        
        if x.len() != y.len() {
            return Err("特征矩阵和目标值长度不匹配".to_string());
        }
        
        let n = x.len();        // 样本数
        let m = x[0].len() + 1; // 特征数 +1 (截距)
        
        // 添加截距列 (全1列)
        let mut x_with_intercept = vec![vec![0.0; m]; n];
        for i in 0..n {
            x_with_intercept[i][0] = 1.0; // 截距
            x_with_intercept[i][1..].copy_from_slice(&x[i]);
        }
        
        // 计算 X^T * X
        let xt = transpose(&x_with_intercept);
        let xtx = matrix_multiply(&xt, &x_with_intercept);
        
        // 计算 X^T * y
        let xty = matrix_vector_multiply(&xt, y);
        
        // 求解 (X^T * X)^-1 * X^T * y (正规方程)
        match matrix_inverse(&xtx) {
            Ok(xtx_inv) => {
                self.coefficients = matrix_vector_multiply(&xtx_inv, &xty);
                self.trained = true;
                self.sample_size = n;
                
                // 计算统计指标
                self.calculate_metrics(&x_with_intercept, y);
                
                Ok(())
            }
            Err(_) => {
                // 如果矩阵奇异,使用岭回归
                self.train_ridge(&x_with_intercept, y, 0.01) // λ=0.01
            }
        }
    }
    
    /// 岭回归 (Ridge Regression) - 处理多重共线性 (从Go代码提取)
    /// β = (X^T * X + λI)^-1 * X^T * y
    fn train_ridge(&mut self, x: &[Vec<f64>], y: &[f64], lambda: f64) -> Result<(), String> {
        let n = x.len();
        let _m = x[0].len();
        
        // X^T * X
        let xt = transpose(x);
        let mut xtx = matrix_multiply(&xt, x);
        
        // X^T * X + λI
        for (i, row) in xtx.iter_mut().enumerate() {
            row[i] += lambda;
        }
        
        // X^T * y
        let xty = matrix_vector_multiply(&xt, y);
        
        // (X^T * X + λI)^-1
        let xtx_inv = matrix_inverse(&xtx)
            .map_err(|e| format!("Ridge regression matrix inversion failed: {}", e))?;
        
        self.coefficients = matrix_vector_multiply(&xtx_inv, &xty);
        self.trained = true;
        self.sample_size = n;
        
        self.calculate_metrics(x, y);
        
        Ok(())
    }
    
    /// 预测 (从Go代码提取)
    pub fn predict(&self, features: &[f64]) -> Result<f64, String> {
        if !self.trained {
            return Err("模型未训练".to_string());
        }
        
        if features.len() + 1 != self.coefficients.len() {
            return Err(format!(
                "特征维度不匹配: 期望{},实际{}",
                self.coefficients.len() - 1,
                features.len()
            ));
        }
        
        // y = β0 + β1*x1 + β2*x2 + ...
        let mut prediction = self.coefficients[0]; // β0 截距
        for (i, &feature) in features.iter().enumerate() {
            prediction += self.coefficients[i + 1] * feature;
        }
        
        Ok(prediction)
    }
    
    /// 计算统计指标 (从Go代码提取)
    fn calculate_metrics(&mut self, x: &[Vec<f64>], y: &[f64]) {
        let n = y.len();
        
        // 计算预测值
        let mut predictions = vec![0.0; n];
        for i in 0..n {
            let mut pred = self.coefficients[0];
            for j in 1..self.coefficients.len() {
                pred += self.coefficients[j] * x[i][j];
            }
            predictions[i] = pred;
        }
        
        // 计算均值
        let y_mean: f64 = y.iter().sum::<f64>() / n as f64;
        
        // 计算SST (总平方和) 和 SSE (误差平方和)
        let mut sst = 0.0;
        let mut sse = 0.0;
        for i in 0..n {
            sst += (y[i] - y_mean).powi(2);
            sse += (y[i] - predictions[i]).powi(2);
        }
        
        // R² = 1 - SSE/SST
        self.r_squared = 1.0 - sse / sst;
        
        // MSE = SSE/n
        self.mse = sse / n as f64;
    }
    
    /// 获取回归系数
    pub fn get_coefficients(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        for (i, name) in self.feature_names.iter().enumerate() {
            if i < self.coefficients.len() {
                result.insert(name.clone(), self.coefficients[i]);
            }
        }
        result
    }
    
    /// 获取模型评估指标
    pub fn get_metrics(&self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        metrics.insert("r_squared".to_string(), self.r_squared);
        metrics.insert("mse".to_string(), self.mse);
        metrics.insert("rmse".to_string(), self.mse.sqrt());
        metrics.insert("sample_size".to_string(), self.sample_size as f64);
        metrics
    }
    
    /// 是否已训练
    pub fn is_trained(&self) -> bool {
        self.trained
    }
}

impl Default for MultipleLinearRegression {
    fn default() -> Self {
        Self::new()
    }
}

// --- 矩阵运算辅助函数 (从Go代码提取) ---

/// 转置矩阵
fn transpose(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![0.0; rows]; cols];
    for (i, col) in result.iter_mut().enumerate() {
        for (j, cell) in col.iter_mut().enumerate() {
            *cell = matrix[j][i];
        }
    }
    result
}

/// 矩阵乘法 A * B
fn matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();
    
    let mut result = vec![vec![0.0; cols_b]; rows_a];
    for (i, row) in result.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let mut sum = 0.0;
            for (k, a_val) in a[i].iter().enumerate().take(cols_a) {
                sum += a_val * b[k][j];
            }
            *cell = sum;
        }
    }
    result
}

/// 矩阵向量乘法 A * v
fn matrix_vector_multiply(a: &[Vec<f64>], v: &[f64]) -> Vec<f64> {
    let rows = a.len();
    let mut result = vec![0.0; rows];
    for (i, row) in a.iter().enumerate() {
        let mut sum = 0.0;
        for (j, &a_val) in row.iter().enumerate() {
            sum += a_val * v[j];
        }
        result[i] = sum;
    }
    result
}

/// 矩阵求逆 (高斯-若尔当消元法) (从Go代码提取)
fn matrix_inverse(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let n = matrix.len();
    
    // 创建增广矩阵 [A | I]
    let mut augmented = vec![vec![0.0; 2 * n]; n];
    for i in 0..n {
        augmented[i][..n].copy_from_slice(&matrix[i]);
        augmented[i][n + i] = 1.0;
    }
    
    // 高斯-若尔当消元
    for i in 0..n {
        // 寻找主元
        let mut max_row = i;
        for k in (i + 1)..n {
            if augmented[k][i].abs() > augmented[max_row][i].abs() {
                max_row = k;
            }
        }
        
        // 交换行
        augmented.swap(i, max_row);
        
        // 检查奇异矩阵
        if augmented[i][i].abs() < 1e-10 {
            return Err("矩阵奇异,无法求逆".to_string());
        }
        
        // 归一化当前行
        let pivot = augmented[i][i];
        for j in 0..(2 * n) {
            augmented[i][j] /= pivot;
        }
        
        // 消元
        for k in 0..n {
            if k != i {
                let factor = augmented[k][i];
                for j in 0..(2 * n) {
                    augmented[k][j] -= factor * augmented[i][j];
                }
            }
        }
    }
    
    // 提取逆矩阵
    let mut inverse = vec![vec![0.0; n]; n];
    for i in 0..n {
        inverse[i].copy_from_slice(&augmented[i][n..]);
    }
    
    Ok(inverse)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_linear_regression_simple() {
        let mut model = MultipleLinearRegression::new();
        
        // 简单的线性关系: y = 2x + 1
        let x = vec![
            vec![1.0],
            vec![2.0],
            vec![3.0],
            vec![4.0],
            vec![5.0],
        ];
        let y = vec![3.0, 5.0, 7.0, 9.0, 11.0];
        
        model.train(&x, &y).unwrap();
        
        // 测试预测
        let pred = model.predict(&[6.0]).unwrap();
        assert!((pred - 13.0).abs() < 0.1);
        
        // 检查R²
        let metrics = model.get_metrics();
        assert!(metrics["r_squared"] > 0.99);
    }
    
    #[test]
    fn test_multiple_linear_regression() {
        let mut model = MultipleLinearRegression::new();
        
        // 多元线性关系: y = 2x1 + 3x2 + 1
        let x = vec![
            vec![1.0, 1.0],
            vec![2.0, 2.0],
            vec![3.0, 3.0],
            vec![4.0, 4.0],
            vec![5.0, 5.0],
        ];
        let y = vec![6.0, 11.0, 16.0, 21.0, 26.0];
        
        model.train(&x, &y).unwrap();
        
        // 测试预测
        let pred = model.predict(&[6.0, 6.0]).unwrap();
        assert!((pred - 31.0).abs() < 0.1);
        
        // 检查训练状态
        assert!(model.is_trained());
    }
    
    #[test]
    fn test_matrix_operations() {
        // 测试转置
        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
        ];
        let transposed = transpose(&matrix);
        assert_eq!(transposed.len(), 3);
        assert_eq!(transposed[0].len(), 2);
        assert_eq!(transposed[0][0], 1.0);
        assert_eq!(transposed[2][1], 6.0);
        
        // 测试矩阵乘法
        let a = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ];
        let b = vec![
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ];
        let c = matrix_multiply(&a, &b);
        assert_eq!(c[0][0], 19.0);
        assert_eq!(c[1][1], 50.0);
    }
    
    #[test]
    fn test_matrix_inverse() {
        let matrix = vec![
            vec![4.0, 7.0],
            vec![2.0, 6.0],
        ];
        let inv = matrix_inverse(&matrix).unwrap();
        
        // 验证 A * A^-1 = I
        let identity = matrix_multiply(&matrix, &inv);
        assert!((identity[0][0] - 1.0).abs() < 1e-10);
        assert!((identity[1][1] - 1.0).abs() < 1e-10);
        assert!(identity[0][1].abs() < 1e-10);
        assert!(identity[1][0].abs() < 1e-10);
    }
}
