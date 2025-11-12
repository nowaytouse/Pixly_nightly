package ai

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Phase 33.2: 反馈数据库 - 在线学习核心
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// FeedbackDB 反馈数据库
type FeedbackDB struct {
	db *sql.DB
	mu sync.Mutex
}

// FeedbackRecord 反馈记录
type FeedbackRecord struct {
	ID              int64                  `json:"id"`
	RequestID       string                 `json:"request_id"`
	Timestamp       time.Time              `json:"timestamp"`
	ModelType       string                 `json:"model_type"`
	ModelVersion    string                 `json:"model_version"`
	ImagePath       string                 `json:"image_path"`
	InputFeatures   map[string]interface{} `json:"input_features"`
	PredictedParams map[string]interface{} `json:"predicted_params"`
	ActualParams    map[string]interface{} `json:"actual_params,omitempty"`
	ActualQuality   float64                `json:"actual_quality,omitempty"`
	ActualSize      int64                  `json:"actual_size,omitempty"`
	UserRating      int                    `json:"user_rating,omitempty"` // 1-5
	ProcessingTime  float64                `json:"processing_time,omitempty"`
	Success         bool                   `json:"success"`
	ErrorMessage    string                 `json:"error_message,omitempty"`
	UsedForTraining bool                   `json:"used_for_training"`
	TrainingBatch   int                    `json:"training_batch,omitempty"`
}

// NewFeedbackDB 创建反馈数据库
func NewFeedbackDB(dbPath string) (*FeedbackDB, error) {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, fmt.Errorf("failed to open database: %v", err)
	}

	fdb := &FeedbackDB{db: db}
	
	// 初始化表结构
	if err := fdb.initTables(); err != nil {
		return nil, err
	}

	Info("FeedbackDB", "Feedback database initialized: %s", dbPath)
	return fdb, nil
}

// initTables 初始化数据表
func (fdb *FeedbackDB) initTables() error {
	schema := `
	CREATE TABLE IF NOT EXISTS feedback (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		request_id TEXT NOT NULL,
		timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
		model_type TEXT NOT NULL,
		model_version TEXT,
		image_path TEXT NOT NULL,
		input_features TEXT,
		predicted_params TEXT,
		actual_params TEXT,
		actual_quality REAL,
		actual_size INTEGER,
		user_rating INTEGER,
		processing_time REAL,
		success BOOLEAN DEFAULT 1,
		error_message TEXT,
		used_for_training BOOLEAN DEFAULT 0,
		training_batch INTEGER,
		INDEX idx_model_type (model_type),
		INDEX idx_timestamp (timestamp),
		INDEX idx_used_for_training (used_for_training)
	);

	CREATE TABLE IF NOT EXISTS training_batches (
		batch_id INTEGER PRIMARY KEY AUTOINCREMENT,
		model_type TEXT NOT NULL,
		start_time DATETIME,
		end_time DATETIME,
		sample_count INTEGER,
		metrics TEXT,
		status TEXT DEFAULT 'pending'
	);

	CREATE TABLE IF NOT EXISTS model_performance (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		model_type TEXT NOT NULL,
		model_version TEXT NOT NULL,
		timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
		avg_quality_error REAL,
		avg_size_error REAL,
		success_rate REAL,
		avg_processing_time REAL,
		sample_count INTEGER
	);
	`

	_, err := fdb.db.Exec(schema)
	return err
}

// RecordFeedback 记录反馈
func (fdb *FeedbackDB) RecordFeedback(record *FeedbackRecord) error {
	fdb.mu.Lock()
	defer fdb.mu.Unlock()

	inputFeaturesJSON, _ := json.Marshal(record.InputFeatures)
	predictedParamsJSON, _ := json.Marshal(record.PredictedParams)
	actualParamsJSON, _ := json.Marshal(record.ActualParams)

	query := `
		INSERT INTO feedback (
			request_id, model_type, model_version, image_path,
			input_features, predicted_params, actual_params,
			actual_quality, actual_size, user_rating,
			processing_time, success, error_message
		) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
	`

	result, err := fdb.db.Exec(query,
		record.RequestID, record.ModelType, record.ModelVersion, record.ImagePath,
		inputFeaturesJSON, predictedParamsJSON, actualParamsJSON,
		record.ActualQuality, record.ActualSize, record.UserRating,
		record.ProcessingTime, record.Success, record.ErrorMessage,
	)

	if err != nil {
		return fmt.Errorf("failed to insert feedback: %v", err)
	}

	id, _ := result.LastInsertId()
	record.ID = id

	return nil
}

// GetUnusedFeedback 获取未用于训练的反馈
func (fdb *FeedbackDB) GetUnusedFeedback(modelType string, limit int) ([]*FeedbackRecord, error) {
	fdb.mu.Lock()
	defer fdb.mu.Unlock()

	query := `
		SELECT id, request_id, timestamp, model_type, model_version,
		       image_path, input_features, predicted_params, actual_params,
		       actual_quality, actual_size, user_rating, processing_time,
		       success, error_message
		FROM feedback
		WHERE model_type = ? AND used_for_training = 0 AND success = 1
		ORDER BY timestamp DESC
		LIMIT ?
	`

	rows, err := fdb.db.Query(query, modelType, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []*FeedbackRecord
	for rows.Next() {
		record := &FeedbackRecord{}
		var inputFeaturesJSON, predictedParamsJSON, actualParamsJSON []byte

		err := rows.Scan(
			&record.ID, &record.RequestID, &record.Timestamp,
			&record.ModelType, &record.ModelVersion, &record.ImagePath,
			&inputFeaturesJSON, &predictedParamsJSON, &actualParamsJSON,
			&record.ActualQuality, &record.ActualSize, &record.UserRating,
			&record.ProcessingTime, &record.Success, &record.ErrorMessage,
		)
		if err != nil {
			continue
		}

		json.Unmarshal(inputFeaturesJSON, &record.InputFeatures)
		json.Unmarshal(predictedParamsJSON, &record.PredictedParams)
		json.Unmarshal(actualParamsJSON, &record.ActualParams)

		records = append(records, record)
	}

	return records, nil
}

// MarkAsUsed 标记反馈已用于训练
func (fdb *FeedbackDB) MarkAsUsed(ids []int64, batchID int) error {
	fdb.mu.Lock()
	defer fdb.mu.Unlock()

	tx, err := fdb.db.Begin()
	if err != nil {
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.Prepare("UPDATE feedback SET used_for_training = 1, training_batch = ? WHERE id = ?")
	if err != nil {
		return err
	}
	defer stmt.Close()

	for _, id := range ids {
		if _, err := stmt.Exec(batchID, id); err != nil {
			return err
		}
	}

	return tx.Commit()
}

// CreateTrainingBatch 创建训练批次
func (fdb *FeedbackDB) CreateTrainingBatch(modelType string, sampleCount int) (int, error) {
	fdb.mu.Lock()
	defer fdb.mu.Unlock()

	query := `
		INSERT INTO training_batches (model_type, start_time, sample_count, status)
		VALUES (?, ?, ?, 'pending')
	`

	result, err := fdb.db.Exec(query, modelType, time.Now(), sampleCount)
	if err != nil {
		return 0, err
	}

	id, _ := result.LastInsertId()
	return int(id), nil
}

// UpdateTrainingBatch 更新训练批次
func (fdb *FeedbackDB) UpdateTrainingBatch(batchID int, metrics map[string]interface{}, status string) error {
	fdb.mu.Lock()
	defer fdb.mu.Unlock()

	metricsJSON, _ := json.Marshal(metrics)

	query := `
		UPDATE training_batches
		SET end_time = ?, metrics = ?, status = ?
		WHERE batch_id = ?
	`

	_, err := fdb.db.Exec(query, time.Now(), metricsJSON, status, batchID)
	return err
}

// RecordModelPerformance 记录模型性能
func (fdb *FeedbackDB) RecordModelPerformance(modelType, modelVersion string, metrics map[string]interface{}) error {
	fdb.mu.Lock()
	defer fdb.mu.Unlock()

	query := `
		INSERT INTO model_performance (
			model_type, model_version, avg_quality_error, avg_size_error,
			success_rate, avg_processing_time, sample_count
		) VALUES (?, ?, ?, ?, ?, ?, ?)
	`

	_, err := fdb.db.Exec(query,
		modelType, modelVersion,
		metrics["avg_quality_error"], metrics["avg_size_error"],
		metrics["success_rate"], metrics["avg_processing_time"],
		metrics["sample_count"],
	)

	return err
}

// GetModelPerformance 获取模型性能历史
func (fdb *FeedbackDB) GetModelPerformance(modelType string, limit int) ([]map[string]interface{}, error) {
	fdb.mu.Lock()
	defer fdb.mu.Unlock()

	query := `
		SELECT model_version, timestamp, avg_quality_error, avg_size_error,
		       success_rate, avg_processing_time, sample_count
		FROM model_performance
		WHERE model_type = ?
		ORDER BY timestamp DESC
		LIMIT ?
	`

	rows, err := fdb.db.Query(query, modelType, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []map[string]interface{}
	for rows.Next() {
		var version string
		var timestamp time.Time
		var avgQualityError, avgSizeError, successRate, avgProcessingTime float64
		var sampleCount int

		if err := rows.Scan(&version, &timestamp, &avgQualityError, &avgSizeError,
			&successRate, &avgProcessingTime, &sampleCount); err != nil {
			continue
		}

		results = append(results, map[string]interface{}{
			"version":              version,
			"timestamp":            timestamp,
			"avg_quality_error":    avgQualityError,
			"avg_size_error":       avgSizeError,
			"success_rate":         successRate,
			"avg_processing_time":  avgProcessingTime,
			"sample_count":         sampleCount,
		})
	}

	return results, nil
}

// GetStats 获取统计信息
func (fdb *FeedbackDB) GetStats() (map[string]interface{}, error) {
	fdb.mu.Lock()
	defer fdb.mu.Unlock()

	stats := make(map[string]interface{})

	// 总记录数
	var totalRecords int
	fdb.db.QueryRow("SELECT COUNT(*) FROM feedback").Scan(&totalRecords)
	stats["total_records"] = totalRecords

	// 未使用的记录数
	var unusedRecords int
	fdb.db.QueryRow("SELECT COUNT(*) FROM feedback WHERE used_for_training = 0").Scan(&unusedRecords)
	stats["unused_records"] = unusedRecords

	// 训练批次数
	var totalBatches int
	fdb.db.QueryRow("SELECT COUNT(*) FROM training_batches").Scan(&totalBatches)
	stats["total_batches"] = totalBatches

	// 按模型类型统计
	rows, _ := fdb.db.Query(`
		SELECT model_type, COUNT(*) as count, 
		       AVG(CASE WHEN success = 1 THEN 1.0 ELSE 0.0 END) as success_rate
		FROM feedback
		GROUP BY model_type
	`)
	defer rows.Close()

	byModel := make(map[string]interface{})
	for rows.Next() {
		var modelType string
		var count int
		var successRate float64
		rows.Scan(&modelType, &count, &successRate)
		byModel[modelType] = map[string]interface{}{
			"count":        count,
			"success_rate": successRate,
		}
	}
	stats["by_model"] = byModel

	return stats, nil
}

// Close 关闭数据库
func (fdb *FeedbackDB) Close() error {
	return fdb.db.Close()
}
