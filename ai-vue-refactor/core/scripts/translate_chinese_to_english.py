#!/usr/bin/env python3
"""
精准翻译Python文件中的中文输出为英文
- 只替换print()中的中文字符串
- 保留代码逻辑和注释
- 保留emoji和格式
"""

import re
import sys
from pathlib import Path

# 精确的中英文对照表（保持技术术语一致性）
TRANSLATIONS = {
    # 状态和结果
    "未安装": "not installed",
    "已就绪": "ready",
    "成功": "success",
    "失败": "failed",
    "完成": "completed",
    "已保存": "saved",
    "已取消": "cancelled",
    "跳过": "skipped",
    
    # 文件和数据
    "文件": "files",
    "个": "",
    "总样本数": "Total samples",
    "图像样本": "Image samples",
    "视频样本": "Video samples",
    "音频样本": "Audio samples",
    "图像": "Images",
    "视频": "Videos",
    "音频": "Audio",
    "总计": "Total",
    "样本数": "Samples",
    
    # 操作
    "扫描媒体文件": "Scanning media files",
    "处理": "Processing",
    "训练数据已保存": "Training data saved",
    "开始全量训练": "Starting full training",
    "自动继续全量训练": "Auto-continuing full training",
    "全量训练已取消": "Full training cancelled",
    
    # 统计
    "文件统计": "File statistics",
    "训练统计": "Training statistics",
    "平均压缩率": "Average compression ratio",
    "平均奖励": "Average reward",
    
    # 步骤
    "步骤": "Step",
    "小规模测试": "Small-scale test",
    "每种类型": "per type",
    "全量训练": "Full training",
    
    # 提示
    "没有找到": "No",
    "测试失败：没有生成训练数据": "Test failed: No training data generated",
    "小规模测试成功！生成了": "Small-scale test succeeded! Generated",
    "个训练样本": "training samples",
    "训练完成": "Training completed",
}

def translate_string(text):
    """翻译单个字符串，保留emoji和格式"""
    result = text
    for zh, en in TRANSLATIONS.items():
        result = result.replace(zh, en)
    return result

def process_file(filepath):
    """处理单个Python文件"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # 匹配print语句中的字符串（支持单引号和双引号）
        # 只替换包含中文的字符串
        def replace_print_string(match):
            quote = match.group(1)
            string_content = match.group(2)
            
            # 检查是否包含中文
            if re.search(r'[\u4e00-\u9fa5]', string_content):
                translated = translate_string(string_content)
                return f'print({quote}{translated}{quote}'
            return match.group(0)
        
        # 替换print中的字符串
        content = re.sub(
            r'print\((["\'])(.*?)\1',
            replace_print_string,
            content
        )
        
        # 替换f-string中的中文
        def replace_fstring(match):
            quote = match.group(1)
            string_content = match.group(2)
            
            if re.search(r'[\u4e00-\u9fa5]', string_content):
                translated = translate_string(string_content)
                return f'print(f{quote}{translated}{quote}'
            return match.group(0)
        
        content = re.sub(
            r'print\(f(["\'])(.*?)\1',
            replace_fstring,
            content
        )
        
        # 只在有变化时写入
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True, "Translated"
        return False, "No changes needed"
        
    except Exception as e:
        return False, f"Error: {str(e)}"

def main():
    if len(sys.argv) < 2:
        print("Usage: python translate_chinese_to_english.py <file_or_directory>")
        sys.exit(1)
    
    target = Path(sys.argv[1])
    
    if target.is_file():
        files = [target]
    elif target.is_dir():
        files = list(target.rglob("*.py"))
    else:
        print(f"Error: {target} not found")
        sys.exit(1)
    
    print(f"Processing {len(files)} Python files...")
    
    translated = 0
    skipped = 0
    errors = 0
    
    for filepath in files:
        changed, message = process_file(filepath)
        if changed:
            print(f"✅ {filepath.relative_to(target.parent if target.is_file() else target)}: {message}")
            translated += 1
        elif "Error" in message:
            print(f"❌ {filepath.relative_to(target.parent if target.is_file() else target)}: {message}")
            errors += 1
        else:
            skipped += 1
    
    print(f"\n📊 Summary:")
    print(f"   Translated: {translated}")
    print(f"   Skipped: {skipped}")
    print(f"   Errors: {errors}")

if __name__ == "__main__":
    main()
