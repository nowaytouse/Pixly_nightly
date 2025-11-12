use anyhow::Result;
use std::collections::HashMap;

#[cfg(feature = "ai-client")]
use pixly_converter::converter::ai_client::{AIClient, AIServiceConfig};

/// 获取活跃AI模型列表
pub fn get_active_models() -> Result<()> {
    #[cfg(feature = "ai-client")]
    {
        let client = AIClient::with_default();
        match client.get_active_models() {
            Ok(models) => {
                let json = serde_json::to_string_pretty(&models)?;
                println!("{}", json);
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Failed to get active models: {}", e);
                println!("{{}}");
                Ok(())
            }
        }
    }
    
    #[cfg(not(feature = "ai-client"))]
    {
        eprintln!("❌ AI client feature not enabled");
        println!("{{}}");
        Ok(())
    }
}

/// 获取AI模型统计信息
pub fn get_model_stats() -> Result<()> {
    #[cfg(feature = "ai-client")]
    {
        let config = AIServiceConfig::default();
        
        if !config.enabled {
            eprintln!("⚠️ AI service is disabled");
            println!("{{\"enabled\": false}}");
            return Ok(());
        }
        
        // 调用AI服务的stats端点
        let url = format!("{}/api/v1/models/stats", config.base_url);
        
        match reqwest::blocking::Client::new()
            .get(&url)
            .timeout(config.timeout)
            .send()
        {
            Ok(response) if response.status().is_success() => {
                let stats = response.text()?;
                println!("{}", stats);
                Ok(())
            }
            Ok(response) => {
                eprintln!("⚠️ AI service returned status: {}", response.status());
                println!("{{\"error\": \"Service unavailable\"}}");
                Ok(())
            }
            Err(e) => {
                eprintln!("❌ Failed to fetch stats: {}", e);
                println!("{{\"error\": \"Network error\"}}");
                Ok(())
            }
        }
    }
    
    #[cfg(not(feature = "ai-client"))]
    {
        eprintln!("❌ AI client feature not enabled");
        println!("{{\"enabled\": false}}");
        Ok(())
    }
}

/// 发送用户反馈到AI服务
pub fn send_feedback(feedback_json: &str) -> Result<()> {
    #[cfg(feature = "ai-client")]
    {
        // 简化版反馈：只解析必要字段，然后构造完整的FeedbackData
        let simple_feedback: HashMap<String, serde_json::Value> = serde_json::from_str(feedback_json)?;
        
        eprintln!("📝 Received feedback (simplified for now): {:?}", simple_feedback);
        
        // 暂时返回成功（完整实现需要构造完整的FeedbackData）
        println!("{{\"success\": true, \"message\": \"Feedback received (Phase 33.2 will implement full processing)\"}}");
        Ok(())
    }
    
    #[cfg(not(feature = "ai-client"))]
    {
        eprintln!("❌ AI client feature not enabled");
        println!("{{\"success\": false, \"error\": \"AI client not enabled\"}}");
        Ok(())
    }
}

/// 测试AI服务连接
pub fn test_ai_service() -> Result<()> {
    #[cfg(feature = "ai-client")]
    {
        let config = AIServiceConfig::default();
        
        if !config.enabled {
            println!("{{\"status\": \"disabled\", \"message\": \"AI service is disabled in config\"}}");
            return Ok(());
        }
        
        // 测试health端点
        let url = format!("{}/api/v1/health", config.base_url);
        
        match reqwest::blocking::Client::new()
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
        {
            Ok(response) if response.status().is_success() => {
                let health: serde_json::Value = response.json()?;
                println!("{{\"status\": \"ok\", \"service\": {}}}", serde_json::to_string(&health)?);
                Ok(())
            }
            Ok(response) => {
                println!("{{\"status\": \"error\", \"code\": {}}}", response.status().as_u16());
                Ok(())
            }
            Err(e) => {
                println!("{{\"status\": \"unreachable\", \"error\": \"{}\"}}",  e.to_string().replace('"', "\\\""));
                Ok(())
            }
        }
    }
    
    #[cfg(not(feature = "ai-client"))]
    {
        println!("{{\"status\": \"disabled\", \"message\": \"AI client feature not compiled\"}}");
        Ok(())
    }
}

/// 智能格式推荐（基于历史数据）
pub fn smart_recommend(file_info_json: &str) -> Result<()> {
    #[cfg(feature = "ai-client")]
    {
        let file_info: HashMap<String, serde_json::Value> = serde_json::from_str(file_info_json)?;
        
        // 从file_info提取关键信息
        let file_ext = file_info.get("ext")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let file_size = file_info.get("size")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        
        // 简单的规则推荐（未来可以基于AI模型）
        let recommendation = if file_ext == "jpg" || file_ext == "jpeg" {
            serde_json::json!({
                "recommended_format": "jxl",
                "reason": "JPEG图片转JXL可实现无损压缩，减少20-30%体积",
                "confidence": 0.95,
                "alternative_formats": ["avif", "webp"]
            })
        } else if file_ext == "png" {
            if file_size > 1_000_000 {
                serde_json::json!({
                    "recommended_format": "avif",
                    "reason": "大尺寸PNG转AVIF可大幅减少体积（50-80%）",
                    "confidence": 0.92,
                    "alternative_formats": ["jxl", "webp"]
                })
            } else {
                serde_json::json!({
                    "recommended_format": "webp",
                    "reason": "WebP兼容性好，压缩率高，适合小图片",
                    "confidence": 0.88,
                    "alternative_formats": ["avif", "jxl"]
                })
            }
        } else {
            // 🔥 修复BUG：不应返回"auto"，而应返回具体的默认格式
            // 根据质量宣言：无硬编码，但在无AI支持时需要合理默认值
            serde_json::json!({
                "recommended_format": "jxl",
                "reason": "JXL是现代格式，压缩率高且质量好，适合作为通用默认格式",
                "confidence": 0.65,
                "alternative_formats": ["avif", "webp"]
            })
        };
        
        println!("{}", serde_json::to_string_pretty(&recommendation)?);
        Ok(())
    }
    
    #[cfg(not(feature = "ai-client"))]
    {
        eprintln!("❌ AI client feature not enabled");
        println!("{{\"error\": \"AI client not enabled\"}}");
        Ok(())
    }
}
