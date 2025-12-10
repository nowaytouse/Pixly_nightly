#!/bin/bash
# Fix Chinese output in shell scripts
# PROJECT_QUALITY_MANIFESTO.md compliance

# Translation map (Chinese → English)
declare -A TRANS=(
    # Status
    ["检查"]="Checking"
    ["安装"]="Installing"
    ["完成"]="Completed"
    ["成功"]="Success"
    ["失败"]="Failed"
    ["跳过"]="Skipped"
    ["警告"]="Warning"
    ["错误"]="Error"
    
    # Actions
    ["开始"]="Starting"
    ["结束"]="Finished"
    ["清理"]="Cleaning"
    ["备份"]="Backing up"
    ["恢复"]="Restoring"
    ["更新"]="Updating"
    ["删除"]="Deleting"
    ["创建"]="Creating"
    ["移动"]="Moving"
    ["复制"]="Copying"
    
    # Environment
    ["环境检查"]="Environment check"
    ["依赖检查"]="Dependency check"
    ["系统信息"]="System info"
    ["配置"]="Configuration"
    
    # Files
    ["文件"]="files"
    ["目录"]="directory"
    ["路径"]="path"
    
    # Common
    ["请"]="Please"
    ["已"]="already"
    ["未"]="not"
    ["正在"]="Currently"
    ["将"]="will"
)

# Process each shell script
for file in scripts/*.sh; do
    if [ -f "$file" ]; then
        # Check if file has Chinese
        if grep -q "[\u4e00-\u9fa5]" "$file" 2>/dev/null; then
            echo "Processing: $file"
            
            # Create backup
            cp "$file" "$file.bak"
            
            # Apply translations
            for zh in "${!TRANS[@]}"; do
                en="${TRANS[$zh]}"
                sed -i '' "s/$zh/$en/g" "$file" 2>/dev/null || sed -i "s/$zh/$en/g" "$file"
            done
            
            echo "  ✅ Translated"
        fi
    fi
done

echo ""
echo "📊 Summary: Shell scripts translation completed"
echo "   Backup files created with .bak extension"
echo "   Review changes and remove .bak files if satisfied"
