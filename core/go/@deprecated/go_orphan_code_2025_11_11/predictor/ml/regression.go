package ml

import (
	"fmt"
	"math"

	"go.uber.org/zap"
)

// MultipleLinearRegression 多元线性回归模型
// 用于预测: Quality = β0 + β1*FileSize + β2*Resolution + β3*Complexity + ...
type MultipleLinearRegression struct {
	logger       *zap.Logger
	coefficients []float64 // 回归系数 [β0, β1, β2, ...]
	featureNames []string  // 特征名称
	trained      bool

	// 统计指标
	rSquared   float64 // R²决定系数
	mse        float64 // 均方误差
	sampleSize int
}

// NewMultipleLinearRegression 创建多元线性回归模型
func NewMultipleLinearRegression(logger *zap.Logger) *MultipleLinearRegression {
	return &MultipleLinearRegression{
		logger:       logger,
		coefficients: nil,
		featureNames: []string{
			"intercept",         // β0 截距
			"file_size_mb",      // β1 文件大小
			"megapixels",        // β2 百万像素
			"complexity",        // β3 复杂度
			"has_alpha",         // β4 透明度
			"is_animated",       // β5 动画
			"estimated_quality", // β6 当前质量
			"noise_level",       // β7 噪声水平
		},
		trained: false,
	}
}

// Train 训练模型 - 最小二乘法
// X: [n_samples, n_features] 特征矩阵
// y: [n_samples] 目标值(如quality)
func (mlr *MultipleLinearRegression) Train(X [][]float64, y []float64) error {
	if len(X) == 0 || len(y) == 0 {
		return fmt.Errorf("训练数据为空")
	}

	if len(X) != len(y) {
		return fmt.Errorf("特征矩阵和目标值长度不匹配")
	}

	n := len(X)        // 样本数
	m := len(X[0]) + 1 // 特征数 +1 (截距)

	// 添加截距列 (全1列)
	XWithIntercept := make([][]float64, n)
	for i := 0; i < n; i++ {
		XWithIntercept[i] = make([]float64, m)
		XWithIntercept[i][0] = 1.0 // 截距
		copy(XWithIntercept[i][1:], X[i])
	}

	// 计算 X^T * X
	XTX := matrixMultiply(transpose(XWithIntercept), XWithIntercept)

	// 计算 X^T * y
	XTy := matrixVectorMultiply(transpose(XWithIntercept), y)

	// 求解 (X^T * X)^-1 * X^T * y (正规方程)
	XTXInv, err := matrixInverse(XTX)
	if err != nil {
		// 如果矩阵奇异,使用岭回归
		mlr.logger.Warn("矩阵奇异,切换到岭回归", zap.Error(err))
		return mlr.trainRidge(XWithIntercept, y, 0.01) // λ=0.01
	}

	mlr.coefficients = matrixVectorMultiply(XTXInv, XTy)
	mlr.trained = true
	mlr.sampleSize = n

	// 计算统计指标
	mlr.calculateMetrics(XWithIntercept, y)

	mlr.logger.Info("多元线性回归训练完成",
		zap.Int("samples", n),
		zap.Int("features", m-1),
		zap.Float64("r_squared", mlr.rSquared),
		zap.Float64("mse", mlr.mse))

	return nil
}

// trainRidge 岭回归 (Ridge Regression) - 处理多重共线性
// β = (X^T * X + λI)^-1 * X^T * y
func (mlr *MultipleLinearRegression) trainRidge(X [][]float64, y []float64, lambda float64) error {
	n := len(X)
	m := len(X[0])

	// X^T * X
	XTX := matrixMultiply(transpose(X), X)

	// X^T * X + λI
	for i := 0; i < m; i++ {
		XTX[i][i] += lambda
	}

	// X^T * y
	XTy := matrixVectorMultiply(transpose(X), y)

	// (X^T * X + λI)^-1
	XTXInv, err := matrixInverse(XTX)
	if err != nil {
		return fmt.Errorf("岭回归矩阵求逆失败: %w", err)
	}

	mlr.coefficients = matrixVectorMultiply(XTXInv, XTy)
	mlr.trained = true
	mlr.sampleSize = n

	mlr.calculateMetrics(X, y)

	mlr.logger.Info("岭回归训练完成",
		zap.Float64("lambda", lambda),
		zap.Float64("r_squared", mlr.rSquared))

	return nil
}

// Predict 预测
func (mlr *MultipleLinearRegression) Predict(features []float64) (float64, error) {
	if !mlr.trained {
		return 0, fmt.Errorf("模型未训练")
	}

	if len(features)+1 != len(mlr.coefficients) {
		return 0, fmt.Errorf("特征维度不匹配: 期望%d,实际%d",
			len(mlr.coefficients)-1, len(features))
	}

	// y = β0 + β1*x1 + β2*x2 + ...
	prediction := mlr.coefficients[0] // β0 截距
	for i, feature := range features {
		prediction += mlr.coefficients[i+1] * feature
	}

	return prediction, nil
}

// calculateMetrics 计算统计指标
func (mlr *MultipleLinearRegression) calculateMetrics(X [][]float64, y []float64) {
	n := len(y)

	// 计算预测值
	predictions := make([]float64, n)
	for i := 0; i < n; i++ {
		pred := mlr.coefficients[0]
		for j := 1; j < len(mlr.coefficients); j++ {
			pred += mlr.coefficients[j] * X[i][j]
		}
		predictions[i] = pred
	}

	// 计算均值
	yMean := 0.0
	for _, val := range y {
		yMean += val
	}
	yMean /= float64(n)

	// 计算SST (总平方和) 和 SSE (误差平方和)
	sst := 0.0
	sse := 0.0
	for i := 0; i < n; i++ {
		sst += math.Pow(y[i]-yMean, 2)
		sse += math.Pow(y[i]-predictions[i], 2)
	}

	// R² = 1 - SSE/SST
	mlr.rSquared = 1.0 - sse/sst

	// MSE = SSE/n
	mlr.mse = sse / float64(n)
}

// GetCoefficients 获取回归系数
func (mlr *MultipleLinearRegression) GetCoefficients() map[string]float64 {
	result := make(map[string]float64)
	for i, name := range mlr.featureNames {
		if i < len(mlr.coefficients) {
			result[name] = mlr.coefficients[i]
		}
	}
	return result
}

// GetMetrics 获取模型评估指标
func (mlr *MultipleLinearRegression) GetMetrics() map[string]float64 {
	return map[string]float64{
		"r_squared":   mlr.rSquared,
		"mse":         mlr.mse,
		"rmse":        math.Sqrt(mlr.mse),
		"sample_size": float64(mlr.sampleSize),
	}
}

// --- 矩阵运算辅助函数 ---

// transpose 转置矩阵
func transpose(matrix [][]float64) [][]float64 {
	rows := len(matrix)
	cols := len(matrix[0])
	result := make([][]float64, cols)
	for i := 0; i < cols; i++ {
		result[i] = make([]float64, rows)
		for j := 0; j < rows; j++ {
			result[i][j] = matrix[j][i]
		}
	}
	return result
}

// matrixMultiply 矩阵乘法 A * B
func matrixMultiply(A, B [][]float64) [][]float64 {
	rowsA := len(A)
	colsA := len(A[0])
	colsB := len(B[0])

	result := make([][]float64, rowsA)
	for i := 0; i < rowsA; i++ {
		result[i] = make([]float64, colsB)
		for j := 0; j < colsB; j++ {
			sum := 0.0
			for k := 0; k < colsA; k++ {
				sum += A[i][k] * B[k][j]
			}
			result[i][j] = sum
		}
	}
	return result
}

// matrixVectorMultiply 矩阵向量乘法 A * v
func matrixVectorMultiply(A [][]float64, v []float64) []float64 {
	rows := len(A)
	result := make([]float64, rows)
	for i := 0; i < rows; i++ {
		sum := 0.0
		for j := 0; j < len(A[i]); j++ {
			sum += A[i][j] * v[j]
		}
		result[i] = sum
	}
	return result
}

// matrixInverse 矩阵求逆 (高斯-若尔当消元法)
func matrixInverse(matrix [][]float64) ([][]float64, error) {
	n := len(matrix)

	// 创建增广矩阵 [A | I]
	augmented := make([][]float64, n)
	for i := 0; i < n; i++ {
		augmented[i] = make([]float64, 2*n)
		copy(augmented[i], matrix[i])
		augmented[i][n+i] = 1.0
	}

	// 高斯-若尔当消元
	for i := 0; i < n; i++ {
		// 寻找主元
		maxRow := i
		for k := i + 1; k < n; k++ {
			if math.Abs(augmented[k][i]) > math.Abs(augmented[maxRow][i]) {
				maxRow = k
			}
		}

		// 交换行
		augmented[i], augmented[maxRow] = augmented[maxRow], augmented[i]

		// 检查奇异矩阵
		if math.Abs(augmented[i][i]) < 1e-10 {
			return nil, fmt.Errorf("矩阵奇异,无法求逆")
		}

		// 归一化当前行
		pivot := augmented[i][i]
		for j := 0; j < 2*n; j++ {
			augmented[i][j] /= pivot
		}

		// 消元
		for k := 0; k < n; k++ {
			if k != i {
				factor := augmented[k][i]
				for j := 0; j < 2*n; j++ {
					augmented[k][j] -= factor * augmented[i][j]
				}
			}
		}
	}

	// 提取逆矩阵
	inverse := make([][]float64, n)
	for i := 0; i < n; i++ {
		inverse[i] = make([]float64, n)
		copy(inverse[i], augmented[i][n:])
	}

	return inverse, nil
}
