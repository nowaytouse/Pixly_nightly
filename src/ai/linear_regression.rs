// 📊 multielementlineregression - from Go MLextraction
// forprediction: Quality = β0 + β1*File Size + β2*Resolution + β3*Complexity + ...
//
// Corealgorithm (from Goextraction):
// - minimumtwo (Ordinary Least Squares)
// - regression (Ridge Regression) - processingmultiheavyline
// - R²calculation
// - Mean squared error (MSE) calculation

use std::collections::HashMap;

/// multielementlineregressionmodel (from Go Multiple Linear Regressionextraction)
pub struct MultipleLinearRegression {
 coefficients: Vec<f64>, // regression [β0, β1, β2, ...]
 feature_names: Vec<String>, // featurename
 trained: bool,

// statistics
 r_squared: f64, // R²
 mse: f64, // Mean squared error
 sample_size: usize,
}

impl MultipleLinearRegression {
/// createnewmultielementlineregressionmodel
 pub fn new() -> Self {
 Self {
 coefficients: Vec::new(),
 feature_names: vec![
 "intercept".to_string(), // β0 
 "file_size_mb".to_string(), // β1 filesize
 "megapixels".to_string(), // β2 hundredten thousandpixel
 "complexity".to_string(), // β3 complexity
 "has_alpha".to_string(), // β4 transparency
 "is_animated".to_string(), // β5 
 "estimated_quality".to_string(), // β6 whenbeforequality
 "noise_level".to_string(), // β7 noise
 ],
 trained: false,
 r_squared: 0.0,
 mse: 0.0,
 sample_size: 0,
 }
 }

/// trainingmodel - minimumtwo (from Goextraction)
///
/// # parameter
/// * `x` - feature \[n_samples, n_features\]
/// * `y` - targetvalue \[n_samples\] (likequality)
 pub fn train(&mut self, x: &[Vec<f64>], y: &[f64]) -> Result<(), String> {
 if x.is_empty() || y.is_empty() {
 return Err("Training data is empty".to_string());
 }

 if x.len() != y.len() {
 return Err("Feature matrix and target vector length mismatch".to_string());
 }

 let n = x.len(); // sample
 let m = x[0].len() + 1; // feature +1 ()

// add (1)
 let mut x_with_intercept = vec![vec![0.0; m]; n];
 for i in 0..n {
 x_with_intercept[i][0] = 1.0; // 
 x_with_intercept[i][1..].copy_from_slice(&x[i]);
 }

// calculation X^T * X
 let xt = transpose(&x_with_intercept);
 let xtx = matrix_multiply(&xt, &x_with_intercept);

// calculation X^T * y
 let xty = matrix_vector_multiply(&xt, y);

//  (X^T * X)^-1 * X^T * y (positive)
 match matrix_inverse(&xtx) {
 Ok(xtx_inv) => {
 self.coefficients = matrix_vector_multiply(&xtx_inv, &xty);
 self.trained = true;
 self.sample_size = n;

// calculationstatistics
 self.calculate_metrics(&x_with_intercept, y);

 Ok(())
 }
 Err(_) => {
// if奇异,useregression
 self.train_ridge(&x_with_intercept, y, 0.01) // λ=0.01
 }
 }
 }

/// regression (Ridge Regression) - processingmultiheavyline (from Goextraction)
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

/// prediction (from Goextraction)
 pub fn predict(&self, features: &[f64]) -> Result<f64, String> {
 if !self.trained {
 return Err("Model not trained".to_string());
 }

 if features.len() + 1 != self.coefficients.len() {
 return Err(format!(
 "Feature dimension mismatch: expected {}, got {}",
 self.coefficients.len() - 1,
 features.len()
 ));
 }

// y = β0 + β1*x1 + β2*x2 + ...
 let mut prediction = self.coefficients[0]; // β0 
 for (i, &feature) in features.iter().enumerate() {
 prediction += self.coefficients[i + 1] * feature;
 }

 Ok(prediction)
 }

/// calculationstatistics (from Goextraction)
 fn calculate_metrics(&mut self, x: &[Vec<f64>], y: &[f64]) {
 let n = y.len();

// calculationpredictionvalue
 let mut predictions = vec![0.0; n];
 for i in 0..n {
 let mut pred = self.coefficients[0];
 for j in 1..self.coefficients.len() {
 pred += self.coefficients[j] * x[i][j];
 }
 predictions[i] = pred;
 }

// calculationvalue
 let y_mean: f64 = y.iter().sum::<f64>() / n as f64;

// calculation SST ( and ) and SSE (error and )
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

/// getregression
 pub fn get_coefficients(&self) -> HashMap<String, f64> {
 let mut result = HashMap::new();
 for (i, name) in self.feature_names.iter().enumerate() {
 if i < self.coefficients.len() {
 result.insert(name.clone(), self.coefficients[i]);
 }
 }
 result
 }

/// getmodelevaluate
 pub fn get_metrics(&self) -> HashMap<String, f64> {
 let mut metrics = HashMap::new();
 metrics.insert("r_squared".to_string(), self.r_squared);
 metrics.insert("mse".to_string(), self.mse);
 metrics.insert("rmse".to_string(), self.mse.sqrt());
 metrics.insert("sample_size".to_string(), self.sample_size as f64);
 metrics
 }

/// isnoalreadytraining
 pub fn is_trained(&self) -> bool {
 self.trained
 }
}

impl Default for MultipleLinearRegression {
 fn default() -> Self {
 Self::new()
 }
}

// --- 运算helperfunction (from Goextraction) ---

/// 
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

///  A * B
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

/// 向量 A * v
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

/// 求逆 (high-ifwhenelement) (from Goextraction)
fn matrix_inverse(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
 let n = matrix.len();

// create增广 [A | I]
 let mut augmented = vec![vec![0.0; 2 * n]; n];
 for i in 0..n {
 augmented[i][..n].copy_from_slice(&matrix[i]);
 augmented[i][n + i] = 1.0;
 }

// high-ifwhenelement
 for i in 0..n {
// findmainelement
 let mut max_row = i;
 for k in (i + 1)..n {
 if augmented[k][i].abs() > augmented[max_row][i].abs() {
 max_row = k;
 }
 }

// line
 augmented.swap(i, max_row);

// check奇异
 if augmented[i][i].abs() < 1e-10 {
 return Err("Matrix is singular, cannot invert".to_string());
 }

// normalizecurrentline
 let pivot = augmented[i][i];
 for j in 0..(2 * n) {
 augmented[i][j] /= pivot;
 }

// element
 for k in 0..n {
 if k != i {
 let factor = augmented[k][i];
 for j in 0..(2 * n) {
 augmented[k][j] -= factor * augmented[i][j];
 }
 }
 }
 }

// extraction逆
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

// singleline关系: y = 2x + 1
 let x = vec![
 vec![1.0],
 vec![2.0],
 vec![3.0],
 vec![4.0],
 vec![5.0],
 ];
 let y = vec![3.0, 5.0, 7.0, 9.0, 11.0];

 model.train(&x, &y).unwrap();

// testprediction
 let pred = model.predict(&[6.0]).unwrap();
 assert!((pred - 13.0).abs() < 0.1);

// check R²
 let metrics = model.get_metrics();
 assert!(metrics["r_squared"] > 0.99);
 }

 #[test]
 fn test_multiple_linear_regression() {
 let mut model = MultipleLinearRegression::new();

// multielementline关系: y = 2x1 + 3x2 + 1
 let x = vec![
 vec![1.0, 1.0],
 vec![2.0, 2.0],
 vec![3.0, 3.0],
 vec![4.0, 4.0],
 vec![5.0, 5.0],
 ];
 let y = vec![6.0, 11.0, 16.0, 21.0, 26.0];

 model.train(&x, &y).unwrap();

// testprediction
 let pred = model.predict(&[6.0, 6.0]).unwrap();
 assert!((pred - 31.0).abs() < 0.1);

// checktrainingstatus
 assert!(model.is_trained());
 }

 #[test]
 fn test_matrix_operations() {
// test
 let matrix = vec![
 vec![1.0, 2.0, 3.0],
 vec![4.0, 5.0, 6.0],
 ];
 let transposed = transpose(&matrix);
 assert_eq!(transposed.len(), 3);
 assert_eq!(transposed[0].len(), 2);
 assert_eq!(transposed[0][0], 1.0);
 assert_eq!(transposed[2][1], 6.0);

// test
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

// validation A * A^-1 = I
 let identity = matrix_multiply(&matrix, &inv);
 assert!((identity[0][0] - 1.0).abs() < 1e-10);
 assert!((identity[1][1] - 1.0).abs() < 1e-10);
 assert!(identity[0][1].abs() < 1e-10);
 assert!(identity[1][0].abs() < 1e-10);
 }
}
