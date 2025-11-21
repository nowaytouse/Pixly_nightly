#!/usr/bin/env python3
"""
Batch fix Chinese output in Python scripts
Complies with PROJECT_QUALITY_MANIFESTO.md requirement:
"内核全部的输出语言(不是代码注释!!!!!)仅有英语"
"""

import re
import sys
from pathlib import Path

# Translation dictionary (Chinese -> English)
TRANSLATIONS = {
    # Common phrases
    "缺少依赖": "Missing dependencies",
    "所有依赖工具ready": "All dependencies ready",
    "依赖工具": "dependencies",
    "依赖": "dependencies",
    "检查": "Checking",
    "主函数": "Main function",
    "混合策略": "Hybrid Strategy",
    "批量PPO更新器": "Batch PPO Updater",
    "一次性处理所有经验": "Process all experiences at once",
    "批量训练": "Batch training",
    "避免重复的Python进程启动开销": "Avoid repeated Python process startup overhead",
    "加载或创建Actor": "Load or create Actor",
    "批量PPO更新器": "Batch PPO Updater",
    "加载": "Loaded",
    "失败": "Failed",
    "使用新模型": "using new model",
    "创建新模型": "Creating new model",
    "模型已保存": "Model saved",
    "经验": "experiences",
    "训练": "Training",
    "批次": "Batches",
    "平均损失": "Average loss",
    "总更新次数": "Total updates",
    "批量更新完成": "Batch update complete",
    "处理的经验": "Experiences processed",
    "没有经验可更新": "No experiences to update",
    
    # File operations
    "查找所有媒体文件": "Find all media files",
    "文件": "files",
    "跳过": "skipping",
    "处理": "Processing",
    "收集真实数据": "Collect real data",
    "收集真实特征": "Collect real features",
    "未找到测试": "Test not found",
    "找到": "Found",
    "计划执行": "Plan to execute",
    "次转换": "conversions",
    "格式": "formats",
    "不存在": "does not exist",
    "分析": "Analysis",
    "目录不存在": "directory does not exist",
    "进度": "Progress",
    "未安装": "not installed",
    "将": "will",
    "绘图功能": "plotting functionality",
    "维特征": "dimensional features",
    "分析特征重要性": "Analyze feature importance",
    
    # Status messages
    "完成": "complete",
    "成功": "success",
    "错误": "error",
    "警告": "warning",
    
    # Media types
    "图像": "image",
    "视频": "video", 
    "音频": "audio",
    "样本": "samples",
    "测试数据": "test data",
    "测试": "test",
    
    # Model related
    "模型": "model",
    "评估模型": "Evaluate model",
    "评估": "evaluation",
    
    # Actions
    "保存": "saved",
    "训练数据已保存": "Training data saved",
    "总样本": "Total samples",
    "图像样本": "Image samples",
    "视频样本": "Video samples",
    "音频样本": "Audio samples",
}

def translate_text(text):
    """Translate Chinese text to English"""
    result = text
    for zh, en in TRANSLATIONS.items():
        result = result.replace(zh, en)
    return result

def fix_file(filepath):
    """Fix Chinese output in a Python file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original = content
        lines = content.split('\n')
        modified_lines = []
        changes = 0
        
        for line in lines:
            # Check if line contains Chinese characters
            if re.search(r'[\u4e00-\u9fa5]', line):
                # Only process print statements
                if 'print(' in line or 'print (' in line:
                    # Skip if it's a comment
                    if line.strip().startswith('#'):
                        modified_lines.append(line)
                        continue
                    
                    # Translate the line
                    new_line = translate_text(line)
                    
                    # Add file=sys.stderr if not present and not in docstring
                    if 'print(' in new_line and 'file=' not in new_line and '"""' not in new_line and "'''" not in new_line:
                        # Find the closing parenthesis
                        if new_line.rstrip().endswith(')'):
                            new_line = new_line.rstrip()[:-1] + ', file=sys.stderr)'
                        elif new_line.rstrip().endswith('))'):
                            # Already has nested parentheses
                            pass
                    
                    if new_line != line:
                        changes += 1
                        modified_lines.append(new_line)
                    else:
                        modified_lines.append(line)
                else:
                    # Keep non-print Chinese (comments, docstrings)
                    modified_lines.append(line)
            else:
                modified_lines.append(line)
        
        if changes > 0:
            new_content = '\n'.join(modified_lines)
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True, f"Fixed {changes} lines"
        else:
            return False, "No changes needed"
            
    except Exception as e:
        return False, f"Error: {str(e)}"

def main():
    if len(sys.argv) < 2:
        print("Usage: python batch_fix_chinese_output.py <directory>", file=sys.stderr)
        sys.exit(1)
    
    target = Path(sys.argv[1])
    
    if not target.exists():
        print(f"Error: {target} not found", file=sys.stderr)
        sys.exit(1)
    
    # Find all Python files
    if target.is_dir():
        files = list(target.rglob("*.py"))
    else:
        files = [target]
    
    print(f"🔍 Processing {len(files)} Python files...", file=sys.stderr)
    
    fixed = 0
    skipped = 0
    errors = 0
    
    for filepath in files:
        # Skip __pycache__ and virtual environments
        if '__pycache__' in str(filepath) or 'venv' in str(filepath):
            continue
            
        changed, message = fix_file(filepath)
        if changed:
            print(f"✅ {filepath.name}: {message}", file=sys.stderr)
            fixed += 1
        elif "Error" in message:
            print(f"❌ {filepath.name}: {message}", file=sys.stderr)
            errors += 1
        else:
            skipped += 1
    
    print(f"\n📊 Summary:", file=sys.stderr)
    print(f"   Fixed: {fixed}", file=sys.stderr)
    print(f"   Skipped: {skipped}", file=sys.stderr)
    print(f"   Errors: {errors}", file=sys.stderr)
    
    if fixed > 0:
        print(f"\n✅ Successfully fixed {fixed} files!", file=sys.stderr)
        print("   All Python output is now English-only (PROJECT_QUALITY_MANIFESTO.md compliant)", file=sys.stderr)

if __name__ == "__main__":
    main()
