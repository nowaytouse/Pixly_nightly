package storage

import (
	"database/sql"
	"fmt"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

// ============================================================================
// SQLite数据库持久化 v4.2.0
// ============================================================================

// Observation 观测记录（与ai.Observation兼容）
type Observation struct {
	ID         int64   `json:"id"`
	Tool       string  `json:"tool"`
	TargetMode string  `json:"target_mode"`
	Quality    int     `json:"quality"`
	Distance   float64 `json:"distance"`
	Reward     float64 `json:"reward"`
	SSIM       float64 `json:"ssim"`
	FileSize   int64   `json:"file_size"`
	Timestamp  int64   `json:"timestamp"`
}

// SQLiteStore SQLite存储实现
type SQLiteStore struct {
	db *sql.DB
}

// NewSQLiteStore 创建SQLite存储
func NewSQLiteStore(dbPath string) (*SQLiteStore, error) {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, fmt.Errorf("打开数据库失败: %v", err)
	}

	store := &SQLiteStore{db: db}

	// 初始化表结构
	if err := store.initSchema(); err != nil {
		db.Close()
		return nil, fmt.Errorf("初始化数据库表失败: %v", err)
	}

	return store, nil
}

// initSchema 初始化数据库表结构
func (s *SQLiteStore) initSchema() error {
	schema := `
	CREATE TABLE IF NOT EXISTS observations (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		tool TEXT NOT NULL,
		target_mode TEXT NOT NULL,
		quality INTEGER NOT NULL,
		distance REAL NOT NULL,
		reward REAL NOT NULL,
		ssim REAL NOT NULL,
		file_size INTEGER NOT NULL,
		timestamp INTEGER NOT NULL,
		INDEX idx_tool_mode (tool, target_mode),
		INDEX idx_timestamp (timestamp)
	);
	`

	_, err := s.db.Exec(schema)
	return err
}

// Save 保存观测
func (s *SQLiteStore) Save(obs *Observation) error {
	query := `
		INSERT INTO observations (tool, target_mode, quality, distance, reward, ssim, file_size, timestamp)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)
	`

	result, err := s.db.Exec(query,
		obs.Tool,
		obs.TargetMode,
		obs.Quality,
		obs.Distance,
		obs.Reward,
		obs.SSIM,
		obs.FileSize,
		time.Now().Unix(),
	)

	if err != nil {
		return fmt.Errorf("保存观测失败: %v", err)
	}

	id, _ := result.LastInsertId()
	obs.ID = id

	return nil
}

// Load 加载观测（按工具和模式过滤，最新的优先）
func (s *SQLiteStore) Load(tool, targetMode string, limit int) ([]*Observation, error) {
	query := `
		SELECT id, tool, target_mode, quality, distance, reward, ssim, file_size, timestamp
		FROM observations
		WHERE tool = ? AND target_mode = ?
		ORDER BY timestamp DESC
		LIMIT ?
	`

	rows, err := s.db.Query(query, tool, targetMode, limit)
	if err != nil {
		return nil, fmt.Errorf("加载观测失败: %v", err)
	}
	defer rows.Close()

	observations := make([]*Observation, 0, limit)
	for rows.Next() {
		obs := &Observation{}
		err := rows.Scan(
			&obs.ID, &obs.Tool, &obs.TargetMode,
			&obs.Quality, &obs.Distance, &obs.Reward,
			&obs.SSIM, &obs.FileSize, &obs.Timestamp,
		)
		if err != nil {
			return nil, fmt.Errorf("扫描观测失败: %v", err)
		}
		observations = append(observations, obs)
	}

	return observations, nil
}

// Count 统计观测数量
func (s *SQLiteStore) Count(tool, targetMode string) (int, error) {
	query := `
		SELECT COUNT(*) FROM observations
		WHERE tool = ? AND target_mode = ?
	`

	var count int
	err := s.db.QueryRow(query, tool, targetMode).Scan(&count)
	if err != nil {
		return 0, fmt.Errorf("统计观测失败: %v", err)
	}

	return count, nil
}

// Clear 清除旧观测
func (s *SQLiteStore) Clear(tool string, olderThan int64) error {
	query := `
		DELETE FROM observations
		WHERE tool = ? AND timestamp < ?
	`

	_, err := s.db.Exec(query, tool, olderThan)
	if err != nil {
		return fmt.Errorf("清除旧观测失败: %v", err)
	}

	return nil
}

// GetStatistics 获取统计信息
func (s *SQLiteStore) GetStatistics(tool, targetMode string) (*Statistics, error) {
	query := `
		SELECT
			COUNT(*) as total,
			AVG(reward) as avg_reward,
			AVG(ssim) as avg_ssim,
			AVG(quality) as avg_quality,
			MIN(timestamp) as first_observation,
			MAX(timestamp) as last_observation
		FROM observations
		WHERE tool = ? AND target_mode = ?
	`

	stats := &Statistics{}
	err := s.db.QueryRow(query, tool, targetMode).Scan(
		&stats.Total,
		&stats.AvgReward,
		&stats.AvgSSIM,
		&stats.AvgQuality,
		&stats.FirstObservation,
		&stats.LastObservation,
	)

	if err != nil {
		return nil, fmt.Errorf("获取统计信息失败: %v", err)
	}

	return stats, nil
}

// GetTopPerforming 获取表现最佳的N个观测
func (s *SQLiteStore) GetTopPerforming(tool, targetMode string, limit int) ([]*Observation, error) {
	query := `
		SELECT id, tool, target_mode, quality, distance, reward, ssim, file_size, timestamp
		FROM observations
		WHERE tool = ? AND target_mode = ?
		ORDER BY reward DESC
		LIMIT ?
	`

	rows, err := s.db.Query(query, tool, targetMode, limit)
	if err != nil {
		return nil, fmt.Errorf("加载表现最佳观测失败: %v", err)
	}
	defer rows.Close()

	observations := make([]*Observation, 0, limit)
	for rows.Next() {
		obs := &Observation{}
		err := rows.Scan(
			&obs.ID, &obs.Tool, &obs.TargetMode,
			&obs.Quality, &obs.Distance, &obs.Reward,
			&obs.SSIM, &obs.FileSize, &obs.Timestamp,
		)
		if err != nil {
			return nil, fmt.Errorf("扫描观测失败: %v", err)
		}
		observations = append(observations, obs)
	}

	return observations, nil
}

// Close 关闭数据库连接
func (s *SQLiteStore) Close() error {
	if s.db != nil {
		return s.db.Close()
	}
	return nil
}

// Statistics 统计信息
type Statistics struct {
	Total            int     `json:"total"`
	AvgReward        float64 `json:"avg_reward"`
	AvgSSIM          float64 `json:"avg_ssim"`
	AvgQuality       float64 `json:"avg_quality"`
	FirstObservation int64   `json:"first_observation"`
	LastObservation  int64   `json:"last_observation"`
}

// ============================================================================
// 观测存储适配器（兼容ai包）
// ============================================================================

// ObservationAdapter SQLite与ai包的观测适配器
type ObservationAdapter struct {
	store *SQLiteStore
}

// NewObservationAdapter 创建适配器
func NewObservationAdapter(dbPath string) (*ObservationAdapter, error) {
	store, err := NewSQLiteStore(dbPath)
	if err != nil {
		return nil, err
	}

	return &ObservationAdapter{store: store}, nil
}

// SaveAIObservation 保存ai包的观测
func (a *ObservationAdapter) SaveAIObservation(tool string, obs interface{}) error {
	// 类型断言和转换
	// 注：实际使用时需要导入ai包并做正确的类型转换
	dbObs := &Observation{
		Tool:       tool,
		TargetMode: "balanced", // 从obs中提取
		// ... 其他字段映射
	}

	return a.store.Save(dbObs)
}

// LoadAIObservations 加载为ai包的观测格式
func (a *ObservationAdapter) LoadAIObservations(tool, targetMode string, limit int) ([]interface{}, error) {
	observations, err := a.store.Load(tool, targetMode, limit)
	if err != nil {
		return nil, err
	}

	// 转换为ai包格式
	result := make([]interface{}, len(observations))
	for i, obs := range observations {
		result[i] = obs // 实际需要类型转换
	}

	return result, nil
}

// Close 关闭适配器
func (a *ObservationAdapter) Close() error {
	return a.store.Close()
}
