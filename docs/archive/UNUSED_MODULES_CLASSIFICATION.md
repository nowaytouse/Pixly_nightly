# 🔍 未使用模块分类报告

**分类日期**: 2025-11-19 17:16:37

## 分类1: 旧架构遗留 (从@archive恢复但已被新架构替代)

- **worker_pool** (150行) - 🗑️ 建议删除
- **visual_quality_scorer** (396行) - 🗑️ 建议删除
- **video_handler** (68行) - 🗑️ 建议删除
- **training_data** (252行) - 🗑️ 建议删除
- **strategy** (204行) - 🗑️ 建议删除
- **smart_concurrency** (243行) - 🗑️ 建议删除
- **ram_optimizer** (314行) - 🗑️ 建议删除
- **quality_predictor** (288行) - 🗑️ 建议删除
- **quality_adjuster** (185行) - 🗑️ 建议删除
- **predictor_core** (190行) - 🗑️ 建议删除
- **parallel** (232行) - 🗑️ 建议删除
- **metadata_processor** (428行) - 🗑️ 建议删除
- **metadata_handler** (82行) - 🗑️ 建议删除
- **memory_manager** (81行) - 🗑️ 建议删除
- **local_ai** (97行) - 🗑️ 建议删除
- **image_analyzer** (306行) - 🗑️ 建议删除
- **gif_processor** (360行) - 🗑️ 建议删除
- **gif_handler** (117行) - 🗑️ 建议删除
- **gif_animation_strategy** (247行) - 🗑️ 建议删除
- **formats** (219行) - 🗑️ 建议删除
- **format_optimizer** (254行) - 🗑️ 建议删除
- **file_detector** (197行) - 🗑️ 建议删除
- **conversion_engine** (193行) - 🗑️ 建议删除
- **conflict_detector** (273行) - 🗑️ 建议删除
- **cli_convert** (1172行) - 🗑️ 建议删除
- **cli_batch** (385行) - 🗑️ 建议删除
- **chroma_predictor** (165行) - 🗑️ 建议删除
- **cache** (520行) - 🗑️ 建议删除
- **batch_processor** (341行) - 🗑️ 建议删除
- **batch_decision** (610行) - 🗑️ 建议删除
- **ai_interface** (234行) - 🗑️ 建议删除

## 分类2: CLI相关模块 (可能被pixly_*.rs使用)

- **cli_main** (774行) - ⚠️ 检查CLI使用情况
- **cli_convert** (1172行) - ⚠️ 检查CLI使用情况
- **cli_batch** (385行) - ⚠️ 检查CLI使用情况

## 分类3: 批处理相关 (可能重复)

- **cli_batch** (385行) - 🤔 评估是否重复
- **batch_processor_advanced** (368行) - 🤔 评估是否重复
- **batch_processor** (341行) - 🤔 评估是否重复
- **batch_decision_manager** (383行) - 🤔 评估是否重复
- **batch_decision** (610行) - 🤔 评估是否重复
- **batch_converter** (502行) - 🤔 评估是否重复
- **batch** (452行) - 🤔 评估是否重复

## 分类4: 高价值功能模块 (应该被使用)

- **format_knowledge** (1047行, 9个公开函数) - 🔥 高价值，需要集成
- **eagle_adapter** (880行, 16个公开函数) - 🔥 高价值，需要集成
- **ai** (804行, 8个公开函数) - 🔥 高价值，需要集成
- **quality_analyzer** (523行, 7个公开函数) - 🔥 高价值，需要集成
- **validation_integration** (445行, 11个公开函数) - 🔥 高价值，需要集成
- **automl** (430行, 7个公开函数) - 🔥 高价值，需要集成
- **visual_quality_scorer** (396行, 3个公开函数) - 🔥 高价值，需要集成
- **ml_predictor** (392行, 8个公开函数) - 🔥 高价值，需要集成

## 分类5: 其他模块

*(剩余模块，需要逐个评估)*
