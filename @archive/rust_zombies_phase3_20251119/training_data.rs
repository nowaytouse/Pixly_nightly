// 训练数据模块 - 从LightGBM训练数据集提取
// 提取自: @archive/go/models/training_dataset.json
// 包含901条真实图像的特征和最优质量参数

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRecord {
    pub image_path: String,
    pub width: u32,
    pub height: u32,
    pub pixels: u32,
    pub aspect_ratio: f64,
    pub has_alpha: u8,
    pub file_size: u64,
    pub edge_strength: f64,
    pub texture_complexity: f64,
    pub noise_level: f64,
    pub detail_level: f64,
    pub high_freq_energy: f64,
    pub mid_freq_energy: f64,
    pub low_freq_energy: f64,
    pub overall_quality: f64,
    pub compression_score: f64,
    pub optimal_quality_jxl: u8,
    pub optimal_quality_avif: u8,
    pub optimal_quality_webp: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingReport {
    pub version: String,
    pub trained_at: String,
    pub dataset_size: usize,
    pub models: TrainingModels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingModels {
    pub jxl: ModelInfo,
    pub avif: ModelInfo,
    pub webp: ModelInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub model_path: String,
    pub metrics: ModelMetrics,
    pub n_samples: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub rmse: f64,
    pub mae: f64,
    pub n_estimators: u32,
}

pub struct TrainingDataset {
    records: Vec<TrainingRecord>,
}

impl TrainingDataset {
    /// 从JSON文件加载训练数据
    pub fn from_json(json_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let records: Vec<TrainingRecord> = serde_json::from_str(json_str)?;
        Ok(Self { records })
    }

    /// 获取所有记录
    pub fn records(&self) -> &[TrainingRecord] {
        &self.records
    }

    /// 根据特征查找相似的训练样本
    pub fn find_similar(&self, width: u32, height: u32, file_size: u64, limit: usize) -> Vec<&TrainingRecord> {
        let mut records_with_distance: Vec<(&TrainingRecord, f64)> = self.records
            .iter()
            .map(|record| {
                let distance = self.calculate_distance(record, width, height, file_size);
                (record, distance)
            })
            .collect();

        // 按距离排序
        records_with_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // 返回最相似的N个
        records_with_distance
            .into_iter()
            .take(limit)
            .map(|(record, _)| record)
            .collect()
    }

    /// 计算特征距离
    fn calculate_distance(&self, record: &TrainingRecord, width: u32, height: u32, file_size: u64) -> f64 {
        let width_diff = (record.width as f64 - width as f64).abs() / width as f64;
        let height_diff = (record.height as f64 - height as f64).abs() / height as f64;
        let size_diff = (record.file_size as f64 - file_size as f64).abs() / file_size as f64;

        // 欧氏距离
        (width_diff.powi(2) + height_diff.powi(2) + size_diff.powi(2)).sqrt()
    }

    /// 根据格式推荐质量参数
    pub fn recommend_quality(&self, format: &str, width: u32, height: u32, file_size: u64) -> Option<u8> {
        let similar = self.find_similar(width, height, file_size, 10);
        if similar.is_empty() {
            return None;
        }

        // 计算平均推荐质量
        let sum: u32 = similar.iter().map(|r| {
            match format {
                "jxl" => r.optimal_quality_jxl as u32,
                "avif" => r.optimal_quality_avif as u32,
                "webp" => r.optimal_quality_webp as u32,
                _ => 85,
            }
        }).sum();

        Some((sum / similar.len() as u32) as u8)
    }

    /// 获取数据集统计信息
    pub fn statistics(&self) -> DatasetStatistics {
        let total = self.records.len();
        
        let avg_quality_jxl = self.records.iter().map(|r| r.optimal_quality_jxl as f64).sum::<f64>() / total as f64;
        let avg_quality_avif = self.records.iter().map(|r| r.optimal_quality_avif as f64).sum::<f64>() / total as f64;
        let avg_quality_webp = self.records.iter().map(|r| r.optimal_quality_webp as f64).sum::<f64>() / total as f64;

        let avg_edge_strength = self.records.iter().map(|r| r.edge_strength).sum::<f64>() / total as f64;
        let avg_texture_complexity = self.records.iter().map(|r| r.texture_complexity).sum::<f64>() / total as f64;

        DatasetStatistics {
            total_records: total,
            avg_quality_jxl,
            avg_quality_avif,
            avg_quality_webp,
            avg_edge_strength,
            avg_texture_complexity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DatasetStatistics {
    pub total_records: usize,
    pub avg_quality_jxl: f64,
    pub avg_quality_avif: f64,
    pub avg_quality_webp: f64,
    pub avg_edge_strength: f64,
    pub avg_texture_complexity: f64,
}

/// 加载训练报告
pub fn load_training_report(json_str: &str) -> Result<TrainingReport, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(json_str)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_training_record_deserialization() {
        let json = r#"{
            "image_path": "test.jpg",
            "width": 1920,
            "height": 1080,
            "pixels": 2073600,
            "aspect_ratio": 1.777,
            "has_alpha": 0,
            "file_size": 500000,
            "edge_strength": 30.0,
            "texture_complexity": 5.0,
            "noise_level": 0.1,
            "detail_level": 0.5,
            "high_freq_energy": 1.0,
            "mid_freq_energy": 0.5,
            "low_freq_energy": 1.0,
            "overall_quality": 85.0,
            "compression_score": 0.95,
            "optimal_quality_jxl": 90,
            "optimal_quality_avif": 85,
            "optimal_quality_webp": 85
        }"#;

        let record: TrainingRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.width, 1920);
        assert_eq!(record.optimal_quality_jxl, 90);
    }

    #[test]
    fn test_dataset_from_json() {
        let json = r#"[{
            "image_path": "test.jpg",
            "width": 1920,
            "height": 1080,
            "pixels": 2073600,
            "aspect_ratio": 1.777,
            "has_alpha": 0,
            "file_size": 500000,
            "edge_strength": 30.0,
            "texture_complexity": 5.0,
            "noise_level": 0.1,
            "detail_level": 0.5,
            "high_freq_energy": 1.0,
            "mid_freq_energy": 0.5,
            "low_freq_energy": 1.0,
            "overall_quality": 85.0,
            "compression_score": 0.95,
            "optimal_quality_jxl": 90,
            "optimal_quality_avif": 85,
            "optimal_quality_webp": 85
        }]"#;

        let dataset = TrainingDataset::from_json(json).unwrap();
        assert_eq!(dataset.records().len(), 1);
    }

    #[test]
    fn test_recommend_quality() {
        let json = r#"[{
            "image_path": "test.jpg",
            "width": 1920,
            "height": 1080,
            "pixels": 2073600,
            "aspect_ratio": 1.777,
            "has_alpha": 0,
            "file_size": 500000,
            "edge_strength": 30.0,
            "texture_complexity": 5.0,
            "noise_level": 0.1,
            "detail_level": 0.5,
            "high_freq_energy": 1.0,
            "mid_freq_energy": 0.5,
            "low_freq_energy": 1.0,
            "overall_quality": 85.0,
            "compression_score": 0.95,
            "optimal_quality_jxl": 90,
            "optimal_quality_avif": 85,
            "optimal_quality_webp": 85
        }]"#;

        let dataset = TrainingDataset::from_json(json).unwrap();
        let quality = dataset.recommend_quality("jxl", 1920, 1080, 500000);
        assert_eq!(quality, Some(90));
    }
}
