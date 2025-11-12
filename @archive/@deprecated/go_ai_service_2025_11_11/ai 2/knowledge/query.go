package knowledge

import (
	"database/sql"
	"fmt"
	"time"

	"go.uber.org/zap"
)

// Query 知识库查询条件
type Query struct {
	// 格式过滤
	SourceFormat string
	TargetFormat string

	// 文件大小范围
	MinFileSize int64
	MaxFileSize int64

	// 分辨率范围
	MinWidth  int
	MaxWidth  int
	MinHeight int
	MaxHeight int

	// 特征过滤
	HasAlpha   *bool // nil=不过滤, true/false=过滤
	IsAnimated *bool

	// 质量过滤
	MinQuality int
	MaxQuality int

	// 时间范围
	StartTime *time.Time
	EndTime   *time.Time

	// 排序和分页
	OrderBy string // "created_at", "file_size", "saving_ratio"
	Limit   int
	Offset  int
}

// QueryRecords 查询转换记录
// 支持复杂条件组合,用于ML学习和统计分析
func (d *Database) QueryRecords(query Query) ([]*ConversionRecord, error) {
	// 构建SQL查询
	sqlQuery := "SELECT * FROM conversion_records WHERE 1=1"
	args := make([]interface{}, 0)

	// 1. 格式过滤
	if query.SourceFormat != "" {
		sqlQuery += " AND original_format = ?"
		args = append(args, query.SourceFormat)
	}
	if query.TargetFormat != "" {
		sqlQuery += " AND target_format = ?"
		args = append(args, query.TargetFormat)
}

	// 2. 文件大小范围
	if query.MinFileSize > 0 {
		sqlQuery += " AND original_size >= ?"
		args = append(args, query.MinFileSize)
	}
	if query.MaxFileSize > 0 {
		sqlQuery += " AND original_size <= ?"
		args = append(args, query.MaxFileSize)
}

	// 3. 分辨率范围
	if query.MinWidth > 0 {
		sqlQuery += " AND width >= ?"
		args = append(args, query.MinWidth)
	}
	if query.MaxWidth > 0 {
		sqlQuery += " AND width <= ?"
		args = append(args, query.MaxWidth)
	}
	if query.MinHeight > 0 {
		sqlQuery += " AND height >= ?"
		args = append(args, query.MinHeight)
	}
	if query.MaxHeight > 0 {
		sqlQuery += " AND height <= ?"
		args = append(args, query.MaxHeight)
}

	// 4. 特征过滤
	if query.HasAlpha != nil {
		sqlQuery += " AND has_alpha = ?"
		args = append(args, *query.HasAlpha)
	}
	if query.IsAnimated != nil {
		sqlQuery += " AND is_animated = ?"
		args = append(args, *query.IsAnimated)
}

	// 5. 质量过滤
	if query.MinQuality > 0 {
		sqlQuery += " AND estimated_quality >= ?"
		args = append(args, query.MinQuality)
	}
	if query.MaxQuality > 0 {
		sqlQuery += " AND estimated_quality <= ?"
		args = append(args, query.MaxQuality)
}

	// 6. 时间范围
	if query.StartTime != nil {
		sqlQuery += " AND created_at >= ?"
		args = append(args, query.StartTime.Unix())
	}
	if query.EndTime != nil {
		sqlQuery += " AND created_at <= ?"
		args = append(args, query.EndTime.Unix())
	}

	// 7. 排序
	orderBy := query.OrderBy
	if orderBy == "" {
		orderBy = "created_at DESC" // 默认按时间倒序
	}
	sqlQuery += " ORDER BY " + orderBy

	// 8. 分页
	if query.Limit > 0 {
		sqlQuery += " LIMIT ?"
		args = append(args, query.Limit)

		if query.Offset > 0 {
			sqlQuery += " OFFSET ?"
			args = append(args, query.Offset)
		}
	}

	// 执行查询
	rows, err := d.db.Query(sqlQuery, args...)
	if err != nil {
		return nil, fmt.Errorf("查询失败: %w", err)
	}
	defer rows.Close()

	records := make([]*ConversionRecord, 0)
	for rows.Next() {
		record := &ConversionRecord{}
		err := rows.Scan(
			&record.ID,
			&record.CreatedAt,
			&record.FilePath,
			&record.FileName,
			&record.OriginalFormat,
			&record.OriginalSize,
			&record.Width,
			&record.Height,
			&record.HasAlpha,
			&record.PixFmt,
			&record.IsAnimated,
			&record.FrameCount,
			&record.EstimatedQuality,
			&record.PredictorName,
			&record.PredictionRule,
			&record.PredictionConfidence,
			&record.PredictionTimeMs,
			&record.PredictedFormat,
			&record.PredictedLossless,
			&record.PredictedDistance,
			&record.PredictedEffort,
			&record.PredictedLosslessJPEG,
			&record.PredictedCRF,
			&record.PredictedSpeed,
			&record.PredictedSavingPercent,
			&record.PredictedOutputSize,
			&record.ActualFormat,
			&record.ActualOutputSize,
			&record.ActualConversionTimeMs,
			&record.ActualSavingPercent,
			&record.ActualSavingBytes,
			&record.ValidationMethod,
			&record.ValidationPassed,
			&record.PixelDiffPercent,
			// 🔧 补齐缺失的8个字段 (42个字段总计)
			&record.PSNRValue,
			&record.SSIMValue,
			&record.PredictionErrorPercent,
			&record.WasExplored,
			&record.UserRating,
			&record.UserComment,
			&record.PixlyVersion,
			&record.HostOS,
		)
		if err != nil {
			d.logger.Warn("扫描记录失败", zap.Error(err))
			continue
		}
		records = append(records, record)
	}

	d.logger.Debug("查询完成",
		zap.Int("result_count", len(records)),
		zap.String("source_format", query.SourceFormat),
		zap.String("target_format", query.TargetFormat))

	return records, nil
}

// QuerySimilarConversions 查询相似转换记录
// 用于ML时间估算和参数优化
func (d *Database) QuerySimilarConversions(
	sourceFormat, targetFormat string,
	fileSize int64,
	width, height int,
	tolerance float64, // 容差比例 (如0.1表示±10%)
	limit int,
) ([]*ConversionRecord, error) {

	sizeTolerance := int64(float64(fileSize) * tolerance)
	widthTolerance := int(float64(width) * tolerance)
	heightTolerance := int(float64(height) * tolerance)

	query := Query{
		SourceFormat: sourceFormat,
		TargetFormat: targetFormat,
		MinFileSize:  fileSize - sizeTolerance,
		MaxFileSize:  fileSize + sizeTolerance,
		MinWidth:     width - widthTolerance,
		MaxWidth:     width + widthTolerance,
		MinHeight:    height - heightTolerance,
		MaxHeight:    height + heightTolerance,
		OrderBy:      "created_at DESC",
		Limit:        limit,
	}

	return d.QueryRecords(query)
}

// GetStatistics 获取统计信息
type Statistics struct {
	TotalRecords       int
	SuccessCount       int
	FailureCount       int
	AvgSavingRatio     float64
	AvgProcessingTime  time.Duration
	FormatDistribution map[string]int
}

func (d *Database) GetStatistics() (*Statistics, error) {
	stats := &Statistics{
		FormatDistribution: make(map[string]int),
	}

	// 总记录数
	err := d.db.QueryRow("SELECT COUNT(*) FROM conversion_records").Scan(&stats.TotalRecords)
	if err != nil {
		return nil, err
}

	// 成功/失败计数
	err = d.db.QueryRow("SELECT COUNT(*) FROM conversion_records WHERE success = 1").Scan(&stats.SuccessCount)
	if err != nil {
		return nil, err
	}
	stats.FailureCount = stats.TotalRecords - stats.SuccessCount

	// 平均节省率
	var avgSaving sql.NullFloat64
	err = d.db.QueryRow("SELECT AVG(saving_ratio) FROM conversion_records WHERE success = 1").Scan(&avgSaving)
	if err == nil && avgSaving.Valid {
		stats.AvgSavingRatio = avgSaving.Float64
	}

	// 平均处理时间
	var avgTime sql.NullInt64
	err = d.db.QueryRow("SELECT AVG(processing_time_ms) FROM conversion_records WHERE success = 1").Scan(&avgTime)
	if err == nil && avgTime.Valid {
		stats.AvgProcessingTime = time.Duration(avgTime.Int64) * time.Millisecond
}

	// 格式分布
	rows, err := d.db.Query(`
		SELECT original_format, COUNT(*) as count 
		FROM conversion_records 
		GROUP BY original_format
		ORDER BY count DESC
	`)
	if err == nil {
		defer rows.Close()
		for rows.Next() {
			var format string
			var count int
			if err := rows.Scan(&format, &count); err == nil {
				stats.FormatDistribution[format] = count
			}
		}
	}

	return stats, nil
}

// GetBestParamsForFormat 获取指定格式组合的最佳参数
// 基于历史成功转换的统计分析
func (d *Database) GetBestParamsForFormat(
	sourceFormat, targetFormat string,
) (quality int, effort int, lossless bool, confidence float64, err error) {

	// 查询成功转换记录,按节省率排序
	query := `
		SELECT predicted_effort, 
		       AVG(actual_saving_percent) as avg_saving, COUNT(*) as count,
		       predicted_lossless
		FROM conversion_records
		WHERE original_format = ? AND predicted_format = ? AND validation_passed = 1
		GROUP BY predicted_effort, predicted_lossless
		HAVING count >= 3
		ORDER BY avg_saving DESC, count DESC
		LIMIT 1
	`

	var avgSaving sql.NullFloat64
	var count int
	var losslessInt int

	err = d.db.QueryRow(query, sourceFormat, targetFormat).Scan(
		&effort, &avgSaving, &count, &losslessInt,
	)

	// 质量从distance推导
	quality = 90 // 默认质量

	if err == sql.ErrNoRows {
		return 0, 0, false, 0, fmt.Errorf("无历史数据")
	}
	if err != nil {
		return 0, 0, false, 0, err
	}

	lossless = (losslessInt == 1)

	// 置信度 = 样本数量 / (样本数量 + 10)
	// 3个样本→0.23, 10个样本→0.5, 30个样本→0.75
	confidence = float64(count) / float64(count+10)

	d.logger.Info("从历史数据获取最佳参数",
		zap.String("format", fmt.Sprintf("%s→%s", sourceFormat, targetFormat)),
		zap.Int("quality", quality),
		zap.Int("effort", effort),
		zap.Bool("lossless", lossless),
		zap.Float64("confidence", confidence),
		zap.Int("sample_count", count))

	return quality, effort, lossless, confidence, nil
}
