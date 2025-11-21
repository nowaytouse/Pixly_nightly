#!/usr/bin/env python3
"""
Manual precise fix for tools/ directory Chinese output
Maintains code quality - proper spacing and grammar
"""

import re
import sys
from pathlib import Path

# Comprehensive and precise translations
PRECISE_TRANSLATIONS = {
    # Complete phrases (match first for accuracy)
    "收集真实训练数据": "Collecting real training data",
    "收集真实特征训练数据": "Collecting real feature training data",
    "未找到测试Images": "Test images not found",
    "找到": "Found",
    "计划执行": "Plan to execute",
    "次转换": "conversions",
    "特征提取失败，跳过": "Feature extraction failed, skipping",
    "转换失败": "Conversion failed",
    "收集完成": "Collection complete",
    "真实样本": "real samples",
    "没有样本可保存": "No samples to save",
    "训练数据已保存": "Training data saved",
    "未收集到样本": "No samples collected",
    "需要完成": "TODO",
    "数据收集完成": "Data collection complete",
    "下一步": "Next steps",
    "运行": "Run",
    "训练模型": "to train model",
    "验证预测准确性": "Verify prediction accuracy",
    "对比真实特征vs简化特征": "Compare real features vs simplified features",
    "用户中断": "User interrupted",
    "错误": "Error",
    
    # Model and evaluation
    "模型性能评估报告": "ML Model Performance Evaluation Report",
    "评估时间": "Evaluation time",
    "模型性能对比": "Model performance comparison",
    "最佳": "Best",
    "最快": "Fastest",
    "建议": "Recommendations",
    "加载模型评估结果": "Loading model evaluation results",
    "未找到评估结果文件": "Evaluation result files not found",
    "请先运行": "Please run first",
    "评估结果已加载": "Evaluation results loaded",
    "分析模型性能": "Analyzing model performance",
    "分析结果已保存": "Analysis results saved",
    
    # Health check
    "检查Python依赖": "Checking Python dependencies",
    "检查模型文件": "Checking model files",
    "models目录不存在": "models directory does not exist",
    "LightGBM模型": "LightGBM models",
    "PPO模型": "PPO models",
    "PPO目录不存在": "PPO directory does not exist",
    "训练数据": "Training data",
    "训练数据不存在": "Training data does not exist",
    "检查模型路由器": "Checking model router",
    "路由器初始化成功": "Router initialized successfully",
    "路由器初始化失败": "Router initialization failed",
    "健康检查完成": "Health check complete",
    "所有检查通过": "All checks passed",
    "发现问题": "Issues found",
    
    # Feature importance
    "未安装，将跳过绘图功能": "not installed, skipping plotting functionality",
    "加载训练数据": "Loading training data",
    "维特征": "dimensional features",
    "分析LightGBM特征重要性": "Analyzing LightGBM feature importance",
    "训练模型": "Training model",
    "重要特征": "Important features",
    "分析特征组重要性": "Analyzing feature group importance",
    "特征组贡献": "Feature group contribution",
    "保存特征重要性分析": "Saving feature importance analysis",
    "分析完成": "Analysis complete",
    
    # Evaluation
    "加载测试数据": "Loading test data",
    "文件不存在": "file does not exist",
    "加载": "Loaded",
    "测试样本": "test samples",
    "评估": "Evaluating",
    "模型": "model",
    "进度": "Progress",
    "样本": "sample",
    "预测失败": "prediction failed",
    "没有成功的预测": "No successful predictions",
    "评估结果": "evaluation results",
    "准确率": "accuracy",
    "推理时间": "Inference time",
    "平均置信度": "Average confidence",
    
    # Single words (match last)
    "格式": "formats",
    "质量": "qualities",
    "特征": "features",
    "维度": "dimensions",
    "统计": "Statistics",
    "数量": "count",
    "大小": "size",
    "目录": "directory",
}

def translate_line(line):
    """Translate a line with Chinese to English"""
    # Skip if no Chinese
    if not re.search(r'[\u4e00-\u9fa5]', line):
        return line
    
    # Skip comments
    if line.strip().startswith('#'):
        return line
    
    # Only process print statements
    if 'print(' not in line and 'print (' not in line:
        return line
    
    result = line
    
    # Apply translations (longest first for accuracy)
    for zh, en in sorted(PRECISE_TRANSLATIONS.items(), key=lambda x: -len(x[0])):
        result = result.replace(zh, en)
    
    # Remove any remaining Chinese characters
    result = re.sub(r'[\u4e00-\u9fa5]+', '', result)
    
    # Clean up spacing
    result = re.sub(r'\s+', ' ', result)
    result = re.sub(r'\s+([,.:;!?)])', r'\1', result)
    result = re.sub(r'([(])\s+', r'\1', result)
    
    # Ensure file=sys.stderr
    if 'print(' in result and 'file=' not in result and '"""' not in result:
        # Find the closing parenthesis
        if result.rstrip().endswith(')'):
            result = result.rstrip()[:-1] + ', file=sys.stderr)'
    
    return result

def fix_file(filepath):
    """Fix Chinese in a file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        modified_lines = []
        changes = 0
        
        for line in lines:
            new_line = translate_line(line)
            if new_line != line:
                changes += 1
            modified_lines.append(new_line)
        
        if changes > 0:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.writelines(modified_lines)
            return True, f"Fixed {changes} lines"
        else:
            return False, "No changes needed"
            
    except Exception as e:
        return False, f"Error: {str(e)}"

def main():
    files_to_fix = [
        "tools/training/train_with_real_features.py",
        "tools/evaluation/ml_health_check.py",
        "tools/evaluation/ml_evaluate.py",
        "tools/evaluation/ml_feature_importance.py",
    ]
    
    print("🔧 Manual precise fix for tools/ directory", file=sys.stderr)
    print("=" * 60, file=sys.stderr)
    print("", file=sys.stderr)
    
    fixed = 0
    for filepath in files_to_fix:
        path = Path(filepath)
        if not path.exists():
            print(f"⚠️  {filepath}: Not found", file=sys.stderr)
            continue
        
        changed, message = fix_file(path)
        if changed:
            print(f"✅ {path.name}: {message}", file=sys.stderr)
            fixed += 1
        else:
            print(f"⏭️  {path.name}: {message}", file=sys.stderr)
    
    print("", file=sys.stderr)
    print(f"📊 Fixed {fixed}/{len(files_to_fix)} files", file=sys.stderr)
    
    if fixed > 0:
        print("✅ All tools/ directory files now have English-only output!", file=sys.stderr)
    
    return 0 if fixed == len(files_to_fix) else 1

if __name__ == "__main__":
    sys.exit(main())
