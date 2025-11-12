// pkg/predictor/ml/features/types.go - 特征类型定义
//
// 功能说明：
// - 定义机器学习特征相关的类型和接口
// - 提供特征向量操作和转换功能
// - 支持特征选择和降维
//
// 作者: AI Assistant
// 版本: v2.2.0
// 更新: 2025-10-24

package features

import (
	"fmt"
	"math"
	"sort"
)

// FeatureVector 特征向量
type FeatureVector struct {
	Values []float64 `json:"values"`
	Names  []string  `json:"names"`
	Dim    int       `json:"dim"`
}

// NewFeatureVector 创建特征向量
func NewFeatureVector(values []float64, names []string) *FeatureVector {
	dim := len(values)
	if names == nil {
		names = make([]string, dim)
		for i := 0; i < dim; i++ {
			names[i] = fmt.Sprintf("feature_%d", i)
		}
	}

	return &FeatureVector{
		Values: values,
		Names:  names,
		Dim:    dim,
	}
}

// GetValue 获取特征值
func (fv *FeatureVector) GetValue(index int) float64 {
	if index < 0 || index >= fv.Dim {
		return 0
	}
	return fv.Values[index]
}

// SetValue 设置特征值
func (fv *FeatureVector) SetValue(index int, value float64) {
	if index >= 0 && index < fv.Dim {
		fv.Values[index] = value
	}
}

// GetName 获取特征名称
func (fv *FeatureVector) GetName(index int) string {
	if index < 0 || index >= fv.Dim {
		return ""
	}
	return fv.Names[index]
}

// Normalize 归一化特征向量
func (fv *FeatureVector) Normalize() {
	// 计算均值和标准差
	mean := fv.Mean()
	std := fv.Std()

	// Z-score归一化
	for i := 0; i < fv.Dim; i++ {
		if std > 0 {
			fv.Values[i] = (fv.Values[i] - mean) / std
		} else {
			fv.Values[i] = 0
		}
	}
}

// MinMaxNormalize 最小-最大归一化
func (fv *FeatureVector) MinMaxNormalize() {
	min, max := fv.MinMax()
	valueRange := max - min

	if valueRange > 0 {
		for i := 0; i < fv.Dim; i++ {
			fv.Values[i] = (fv.Values[i] - min) / valueRange
		}
	}
}

// Mean 计算均值
func (fv *FeatureVector) Mean() float64 {
	sum := 0.0
	for _, v := range fv.Values {
		sum += v
	}
	return sum / float64(fv.Dim)
}

// Std 计算标准差
func (fv *FeatureVector) Std() float64 {
	mean := fv.Mean()
	variance := 0.0

	for _, v := range fv.Values {
		variance += math.Pow(v-mean, 2)
	}
	variance /= float64(fv.Dim)

	return math.Sqrt(variance)
}

// MinMax 计算最小值和最大值
func (fv *FeatureVector) MinMax() (float64, float64) {
	if fv.Dim == 0 {
		return 0, 0
	}

	min := fv.Values[0]
	max := fv.Values[0]

	for _, v := range fv.Values {
		if v < min {
			min = v
		}
		if v > max {
			max = v
		}
	}

	return min, max
}

// DotProduct 计算点积
func (fv *FeatureVector) DotProduct(other *FeatureVector) float64 {
	if fv.Dim != other.Dim {
		return 0
	}

	product := 0.0
	for i := 0; i < fv.Dim; i++ {
		product += fv.Values[i] * other.Values[i]
	}

	return product
}

// EuclideanDistance 计算欧几里得距离
func (fv *FeatureVector) EuclideanDistance(other *FeatureVector) float64 {
	if fv.Dim != other.Dim {
		return math.Inf(1)
	}

	distance := 0.0
	for i := 0; i < fv.Dim; i++ {
		diff := fv.Values[i] - other.Values[i]
		distance += diff * diff
	}

	return math.Sqrt(distance)
}

// CosineSimilarity 计算余弦相似度
func (fv *FeatureVector) CosineSimilarity(other *FeatureVector) float64 {
	dotProduct := fv.DotProduct(other)
	magnitude1 := fv.Magnitude()
	magnitude2 := other.Magnitude()

	if magnitude1 == 0 || magnitude2 == 0 {
		return 0
	}

	return dotProduct / (magnitude1 * magnitude2)
}

// Magnitude 计算向量模长
func (fv *FeatureVector) Magnitude() float64 {
	sum := 0.0
	for _, v := range fv.Values {
		sum += v * v
	}
	return math.Sqrt(sum)
}

// FeatureImportance 特征重要性
type FeatureImportance struct {
	Index      int     `json:"index"`
	Name       string  `json:"name"`
	Importance float64 `json:"importance"`
}

// FeatureSelector 特征选择器
type FeatureSelector struct {
	Importances []FeatureImportance `json:"importances"`
	Threshold   float64             `json:"threshold"`
}

// NewFeatureSelector 创建特征选择器
func NewFeatureSelector(threshold float64) *FeatureSelector {
	return &FeatureSelector{
		Importances: make([]FeatureImportance, 0),
		Threshold:   threshold,
	}
}

// AddImportance 添加特征重要性
func (fs *FeatureSelector) AddImportance(index int, name string, importance float64) {
	fs.Importances = append(fs.Importances, FeatureImportance{
		Index:      index,
		Name:       name,
		Importance: importance,
	})
}

// SelectFeatures 选择重要特征
func (fs *FeatureSelector) SelectFeatures() []int {
	// 按重要性排序
	sort.Slice(fs.Importances, func(i, j int) bool {
		return fs.Importances[i].Importance > fs.Importances[j].Importance
	})

	// 选择超过阈值的特征
	selected := make([]int, 0)
	for _, imp := range fs.Importances {
		if imp.Importance >= fs.Threshold {
			selected = append(selected, imp.Index)
		}
	}

	return selected
}

// GetTopFeatures 获取最重要的特征
func (fs *FeatureSelector) GetTopFeatures(n int) []int {
	// 按重要性排序
	sort.Slice(fs.Importances, func(i, j int) bool {
		return fs.Importances[i].Importance > fs.Importances[j].Importance
	})

	// 选择前n个特征
	selected := make([]int, 0, n)
	for i, imp := range fs.Importances {
		if i >= n {
			break
		}
		selected = append(selected, imp.Index)
	}

	return selected
}

// FeatureMatrix 特征矩阵
type FeatureMatrix struct {
	Data  [][]float64 `json:"data"`
	Names []string    `json:"names"`
	Rows  int         `json:"rows"`
	Cols  int         `json:"cols"`
}

// NewFeatureMatrix 创建特征矩阵
func NewFeatureMatrix(rows, cols int) *FeatureMatrix {
	data := make([][]float64, rows)
	for i := range data {
		data[i] = make([]float64, cols)
	}

	names := make([]string, cols)
	for i := 0; i < cols; i++ {
		names[i] = fmt.Sprintf("feature_%d", i)
	}

	return &FeatureMatrix{
		Data:  data,
		Names: names,
		Rows:  rows,
		Cols:  cols,
	}
}

// SetValue 设置矩阵值
func (fm *FeatureMatrix) SetValue(row, col int, value float64) {
	if row >= 0 && row < fm.Rows && col >= 0 && col < fm.Cols {
		fm.Data[row][col] = value
	}
}

// GetValue 获取矩阵值
func (fm *FeatureMatrix) GetValue(row, col int) float64 {
	if row >= 0 && row < fm.Rows && col >= 0 && col < fm.Cols {
		return fm.Data[row][col]
	}
	return 0
}

// GetRow 获取行向量
func (fm *FeatureMatrix) GetRow(row int) []float64 {
	if row >= 0 && row < fm.Rows {
		return fm.Data[row]
	}
	return nil
}

// GetColumn 获取列向量
func (fm *FeatureMatrix) GetColumn(col int) []float64 {
	if col < 0 || col >= fm.Cols {
		return nil
	}

	column := make([]float64, fm.Rows)
	for i := 0; i < fm.Rows; i++ {
		column[i] = fm.Data[i][col]
	}

	return column
}

// NormalizeColumns 按列归一化
func (fm *FeatureMatrix) NormalizeColumns() {
	for col := 0; col < fm.Cols; col++ {
		column := fm.GetColumn(col)

		// 计算均值和标准差
		mean := 0.0
		for _, v := range column {
			mean += v
		}
		mean /= float64(len(column))

		variance := 0.0
		for _, v := range column {
			variance += math.Pow(v-mean, 2)
		}
		variance /= float64(len(column))
		std := math.Sqrt(variance)

		// Z-score归一化
		if std > 0 {
			for row := 0; row < fm.Rows; row++ {
				fm.Data[row][col] = (fm.Data[row][col] - mean) / std
			}
		}
	}
}

// FeatureStats 特征统计信息
type FeatureStats struct {
	Mean     float64 `json:"mean"`
	Std      float64 `json:"std"`
	Min      float64 `json:"min"`
	Max      float64 `json:"max"`
	Median   float64 `json:"median"`
	Q1       float64 `json:"q1"`
	Q3       float64 `json:"q3"`
	Skewness float64 `json:"skewness"`
	Kurtosis float64 `json:"kurtosis"`
}

// CalculateStats 计算特征统计信息
func CalculateStats(values []float64) *FeatureStats {
	if len(values) == 0 {
		return &FeatureStats{}
	}

	// 排序用于计算中位数和四分位数
	sorted := make([]float64, len(values))
	copy(sorted, values)
	sort.Float64s(sorted)

	stats := &FeatureStats{}
	n := float64(len(values))

	// 计算均值
	sum := 0.0
	for _, v := range values {
		sum += v
	}
	stats.Mean = sum / n

	// 计算标准差
	variance := 0.0
	for _, v := range values {
		variance += math.Pow(v-stats.Mean, 2)
	}
	variance /= n
	stats.Std = math.Sqrt(variance)

	// 计算最小值和最大值
	stats.Min = sorted[0]
	stats.Max = sorted[len(sorted)-1]

	// 计算中位数
	if len(sorted)%2 == 0 {
		stats.Median = (sorted[len(sorted)/2-1] + sorted[len(sorted)/2]) / 2
	} else {
		stats.Median = sorted[len(sorted)/2]
	}

	// 计算四分位数
	q1Index := int(n * 0.25)
	q3Index := int(n * 0.75)
	stats.Q1 = sorted[q1Index]
	stats.Q3 = sorted[q3Index]

	// 计算偏度和峰度（简化实现）
	stats.Skewness = 0.0 // 简化实现
	stats.Kurtosis = 0.0 // 简化实现

	return stats
}
