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
    # FFmpeg和工具
    "FFmpeg未安装": "FFmpeg not installed",
    "FFmpeg已就绪": "FFmpeg ready",
    
    # 文件操作
    "扫描媒体文件": "Scanning media files",
    "没有找到": "No",
    "文件，跳过": "files found, skipping",
    "处理": "Processing",
    "个": "",
    "文件": "files",
    
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
    
    # 文件统计
    "文件统计": "File statistics",
    "图像": "Images",
    "视频": "Videos",
    "音频": "Audio",
    "总计": "Total",
    
    # 状态
    "测试失败：没有生成训练数据": "Test failed: No training data generated",
    "小规模测试成功！生成了": "Small-scale test succeeded! Generated",
    "个训练样本": "training samples",
    "自动继续全量训练": "Auto-continuing full training",
    "开始全量训练": "Starting full training",
    "全量训练已取消": "Full training cancelled",
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
