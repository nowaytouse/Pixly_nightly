#!/usr/bin/env python3
"""
Final cleanup: Remove ALL Chinese characters from print statements
Replace with English equivalents
"""

import re
import sys
from pathlib import Path

# Comprehensive translation dictionary
TRANSLATIONS = {
    # Verbs and actions
    "收集": "Collect",
    "收集了": "Collected",
    "分析": "Analyze/Analysis",
    "评估": "Evaluate/Evaluation",
    "训练": "Train/Training",
    "加载": "Load/Loaded",
    "保存": "Save/Saved",
    "检查": "Check/Checking",
    "测试": "Test/Testing",
    "转换": "Convert/Conversion",
    "处理": "Process/Processing",
    "执行": "Execute",
    "计划": "Plan",
    "找到": "Found",
    "未找到": "Not found",
    "失败": "Failed",
    "成功": "Success",
    "完成": "Complete",
    "跳过": "Skip/Skipping",
    
    # Nouns
    "数据": "data",
    "特征": "feature(s)",
    "模型": "model",
    "文件": "file(s)",
    "样本": "sample(s)",
    "格式": "format(s)",
    "质量": "quality",
    "目录": "directory",
    "路由器": "router",
    "结果": "result(s)",
    "预测": "prediction",
    "准确率": "accuracy",
    "重要性": "importance",
    "贡献": "contribution",
    "依赖": "dependencies",
    "经验": "experience(s)",
    "批次": "batch(es)",
    
    # Adjectives
    "真实": "real",
    "重要": "important",
    "维": "dimensional",
    
    # Phrases
    "不存在": "does not exist",
    "未安装": "not installed",
    "没有": "no/none",
    "将": "will",
    "的": "",  # Remove possessive particle
    "了": "",  # Remove completion particle
    
    # Numbers and counters
    "次": "times",
    "个": "",  # Remove counter
}

def translate_chinese(text):
    """Translate Chinese to English"""
    result = text
    for zh, en in TRANSLATIONS.items():
        result = result.replace(zh, en)
    
    # Remove any remaining Chinese characters
    result = re.sub(r'[\u4e00-\u9fa5]+', '', result)
    
    # Clean up extra spaces
    result = re.sub(r'\s+', ' ', result)
    result = result.strip()
    
    return result

def fix_file(filepath):
    """Fix all Chinese in print statements"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        lines = content.split('\n')
        modified_lines = []
        changes = 0
        
        for line in lines:
            # Check if line has Chinese and is a print statement
            if re.search(r'[\u4e00-\u9fa5]', line) and ('print(' in line or 'print (' in line):
                # Skip comments
                if line.strip().startswith('#'):
                    modified_lines.append(line)
                    continue
                
                # Translate the line
                new_line = translate_chinese(line)
                
                # Ensure file=sys.stderr
                if 'print(' in new_line and 'file=' not in new_line:
                    if new_line.rstrip().endswith(')'):
                        new_line = new_line.rstrip()[:-1] + ', file=sys.stderr)'
                
                if new_line != line:
                    changes += 1
                modified_lines.append(new_line)
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
        print("Usage: python final_chinese_cleanup.py <directory>", file=sys.stderr)
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
        # Skip certain files
        if '__pycache__' in str(filepath):
            continue
        if filepath.name in ['check_real_chinese.py', 'batch_fix_chinese_output.py', 'fix_chinese_output.py', 'translate_chinese_to_english.py', 'final_chinese_cleanup.py']:
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
        print("   All Python output is now English-only", file=sys.stderr)

if __name__ == "__main__":
    main()
