#!/usr/bin/env python3
"""
Check for REAL Chinese characters (not emoji) in Python print statements
"""

import re
import sys
from pathlib import Path

def has_real_chinese(text):
    """Check if text contains real Chinese characters (not emoji)"""
    # Chinese character ranges (excluding emoji and symbols)
    chinese_pattern = r'[\u4e00-\u9fa5]'
    return bool(re.search(chinese_pattern, text))

def check_file(filepath):
    """Check a Python file for Chinese print statements"""
    issues = []
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        for i, line in enumerate(lines, 1):
            # Skip comments
            if line.strip().startswith('#'):
                continue
            
            # Check if it's a print statement with Chinese
            if 'print(' in line or 'print (' in line:
                if has_real_chinese(line):
                    # Extract the Chinese part
                    chinese_chars = ''.join(re.findall(r'[\u4e00-\u9fa5]+', line))
                    issues.append({
                        'line': i,
                        'content': line.strip(),
                        'chinese': chinese_chars
                    })
    
    except Exception as e:
        print(f"Error reading {filepath}: {e}", file=sys.stderr)
    
    return issues

def main():
    if len(sys.argv) < 2:
        print("Usage: python check_real_chinese.py <directory>", file=sys.stderr)
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
    
    print(f"🔍 Checking {len(files)} Python files for REAL Chinese output...\n", file=sys.stderr)
    
    total_issues = 0
    files_with_issues = []
    
    for filepath in files:
        # Skip __pycache__ and certain files
        if '__pycache__' in str(filepath):
            continue
        if filepath.name in ['check_real_chinese.py', 'batch_fix_chinese_output.py', 'fix_chinese_output.py', 'translate_chinese_to_english.py']:
            continue
        
        issues = check_file(filepath)
        if issues:
            files_with_issues.append((filepath, issues))
            total_issues += len(issues)
    
    if files_with_issues:
        print(f"❌ Found {total_issues} print statements with Chinese in {len(files_with_issues)} files:\n", file=sys.stderr)
        
        for filepath, issues in files_with_issues:
            print(f"📄 {filepath.relative_to(target.parent if target.is_file() else target)}:", file=sys.stderr)
            for issue in issues[:5]:  # Show first 5 issues per file
                print(f"   Line {issue['line']}: {issue['chinese']}", file=sys.stderr)
                print(f"      {issue['content'][:80]}...", file=sys.stderr)
            if len(issues) > 5:
                print(f"   ... and {len(issues) - 5} more", file=sys.stderr)
            print("", file=sys.stderr)
        
        return 1
    else:
        print("✅ No Chinese characters found in print statements!", file=sys.stderr)
        return 0

if __name__ == "__main__":
    sys.exit(main())
