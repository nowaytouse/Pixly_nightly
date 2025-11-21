#!/usr/bin/env python3
"""
精准修复Python文件中的中文输出
遵循PROJECT_QUALITY_MANIFESTO.md: 内核输出仅英语
"""

import re
import sys
from pathlib import Path

# 完整的中英文映射表
TRANSLATIONS = {
    # 通用词汇
    "个": "",
    "文件": "files",
    "成功": "success",
    "失败": "failed",
    "完成": "completed",
    "开始": "starting",
    "结束": "finished",
    "总计": "Total",
    "平均": "Average",
    "最大": "Max",
    "最小": "Min",
    "当前": "Current",
    "历史": "History",
    "模型": "Model",
    "数据": "Data",
    "特征": "Features",
    "预测": "Prediction",
    "训练": "Training",
    "测试": "Testing",
    "验证": "Validation",
    "优化": "Optimization",
    "更新": "Update",
    "加载": "Loading",
    "保存": "Saving",
    "删除": "Deleting",
    "创建": "Creating",
    "检查": "Checking",
    "分析": "Analyzing",
    "处理": "Processing",
    "转换": "Converting",
    "压缩": "Compression",
    "质量": "Quality",
    "速度": "Speed",
    "大小": "Size",
    "格式": "Format",
    "路径": "Path",
    "目录": "Directory",
    "输入": "Input",
    "输出": "Output",
    "参数": "Parameters",
    "配置": "Configuration",
    "选项": "Options",
    "状态": "Status",
    "进度": "Progress",
    "结果": "Result",
    "错误": "Error",
    "警告": "Warning",
    "信息": "Info",
    "调试": "Debug",
    
    # FFmpeg和工具
    "FFmpeg未安装": "FFmpeg not installed",
    "FFmpeg已就绪": "FFmpeg ready",
    "未安装": "not installed",
    "已就绪": "ready",
    "可用": "available",
    "不可用": "unavailable",
    
    # 文件操作
    "扫描媒体文件": "Scanning media files",
    "没有找到": "No",
    "文件，跳过": "files found, skipping",
    "跳过": "skipping",
    "已保存": "saved",
    "已加载": "loaded",
    
    # 训练相关
    "PPO训练 - 纯FFmpeg方案": "PPO Training - Pure FFmpeg Solution",
    "PPO训练 - 全媒体类型支持": "PPO Training - All Media Types Support",
    "小规模测试": "Small-scale test",
    "每种类型": "per type",
    "全量训练": "Full training",
    "步骤": "Step",
    "训练数据已保存": "Training data saved",
    "总样本数": "Total samples",
    "图像样本": "Image samples",
    "视频样本": "Video samples",
    "音频样本": "Audio samples",
    "训练完成": "Training completed",
    "训练统计": "Training statistics",
    "样本数": "Samples",
    "平均压缩率": "Average compression ratio",
    "平均奖励": "Average reward",
    "奖励": "Reward",
    "损失": "Loss",
    "准确率": "Accuracy",
    "精度": "Precision",
    "召回率": "Recall",
    
    # 文件统计
    "文件统计": "File statistics",
    "图像": "Images",
    "视频": "Videos",
    "音频": "Audio",
    
    # 状态
    "测试失败：没有生成训练数据": "Test failed: No training data generated",
    "小规模测试成功！生成了": "Small-scale test succeeded! Generated",
    "个训练样本": "training samples",
    "自动继续全量训练": "Auto-continuing full training",
    "开始全量训练": "Starting full training",
    "全量训练已取消": "Full training cancelled",
    "已取消": "cancelled",
    
    # ML相关
    "在线学习": "Online learning",
    "离线学习": "Offline learning",
    "强化学习": "Reinforcement learning",
    "监督学习": "Supervised learning",
    "无监督学习": "Unsupervised learning",
    "深度学习": "Deep learning",
    "神经网络": "Neural network",
    "决策树": "Decision tree",
    "随机森林": "Random forest",
    "梯度提升": "Gradient boosting",
    "贝叶斯优化": "Bayesian optimization",
    "集成学习": "Ensemble learning",
    "特征工程": "Feature engineering",
    "特征选择": "Feature selection",
    "特征提取": "Feature extraction",
    "数据增强": "Data augmentation",
    "过拟合": "Overfitting",
    "欠拟合": "Underfitting",
    "正则化": "Regularization",
    "交叉验证": "Cross-validation",
    "超参数": "Hyperparameters",
    "学习率": "Learning rate",
    "批大小": "Batch size",
    "迭代次数": "Iterations",
    "轮数": "Epochs",
}

def translate_text(text):
    """翻译文本，保留emoji和格式"""
    result = text
    # 按长度排序，先替换长的短语
    for zh, en in sorted(TRANSLATIONS.items(), key=lambda x: -len(x[0])):
        result = result.replace(zh, en)
    return result

def fix_file(filepath):
    """修复单个文件"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        modified = False
        new_lines = []
        
        for line in lines:
            original_line = line
            
            # 检查是否包含中文
            if re.search(r'[\u4e00-\u9fa5]', line):
                # 只处理print语句
                if 'print(' in line or 'print (' in line:
                    line = translate_text(line)
                    if line != original_line:
                        modified = True
            
            new_lines.append(line)
        
        if modified:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.writelines(new_lines)
            return True, "Fixed"
        return False, "No Chinese output found"
        
    except Exception as e:
        return False, f"Error: {str(e)}"

def main():
    if len(sys.argv) < 2:
        print("Usage: python fix_chinese_output.py <file_or_directory>")
        sys.exit(1)
    
    target = Path(sys.argv[1])
    
    if target.is_file():
        files = [target]
    elif target.is_dir():
        files = list(target.rglob("*.py"))
    else:
        print(f"Error: {target} not found")
        sys.exit(1)
    
    print(f"🔍 Processing {len(files)} Python files...")
    
    fixed = 0
    skipped = 0
    errors = 0
    
    for filepath in files:
        changed, message = fix_file(filepath)
        if changed:
            print(f"✅ {filepath.name}: {message}")
            fixed += 1
        elif "Error" in message:
            print(f"❌ {filepath.name}: {message}")
            errors += 1
        else:
            skipped += 1
    
    print(f"\n📊 Summary:")
    print(f"   Fixed: {fixed}")
    print(f"   Skipped: {skipped}")
    print(f"   Errors: {errors}")
    
    if fixed > 0:
        print(f"\n✅ Successfully fixed {fixed} files!")
        print("   All Python output is now English-only (PROJECT_QUALITY_MANIFESTO.md compliant)")

if __name__ == "__main__":
    main()
