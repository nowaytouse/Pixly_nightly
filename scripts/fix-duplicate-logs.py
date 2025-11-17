#!/usr/bin/env python3
"""
自动修复ui-handlers.js中重复的let log声明
保留每个函数的第一个声明，删除后续的重复声明
"""

import re

def fix_duplicate_logs(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    result = []
    in_function = False
    function_start = -1
    first_log_seen = False
    log_pattern = re.compile(r'^\s*let log = window\.pixlyLog \|\| console;')
    function_pattern = re.compile(r'addEventListener.*\(|^function\s+\w+\s*\(|^\s*\w+\s*:\s*function\s*\(|^\s*\w+\s*\([^)]*\)\s*{')
    
    for i, line in enumerate(lines):
        line_num = i + 1
        
        # 检测函数开始
        if function_pattern.search(line):
            in_function = True
            function_start = line_num
            first_log_seen = False
            result.append(line)
            continue
        
        # 检测函数结束（简单检测：大括号闭合）
        if in_function and line.strip() == '});':
            in_function = False
            first_log_seen = False
            result.append(line)
            continue
        
        # 检测let log声明
        if log_pattern.match(line):
            if not first_log_seen:
                # 第一次出现，保留
                first_log_seen = True
                result.append(line)
            else:
                # 重复出现，替换为注释
                indent = len(line) - len(line.lstrip())
                comment = ' ' * indent + f'// log已在函数开头声明，无需重复 (原行{line_num})\n'
                result.append(comment)
                print(f'Line {line_num}: Removed duplicate let log')
        else:
            result.append(line)
    
    # 写回文件
    with open(filepath, 'w', encoding='utf-8') as f:
        f.writelines(result)
    
    print(f'\nFixed {filepath}')

if __name__ == '__main__':
    filepath = '/Users/nyamiiko/Documents/git/Pixly/Pixly_Nightly/core/plugin/js/plugin-modules/ui-handlers.js'
    fix_duplicate_logs(filepath)
