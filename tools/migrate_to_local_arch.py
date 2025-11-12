#!/usr/bin/env python3
"""
网络架构到本地架构迁移工具

彻底摆脱网络依赖，实现真正的本地化：
- 自动检测HTTP调用
- 替换为直接函数调用
- 消除序列化开销
- 零网络依赖架构

使用方法:
    python tools/migrate_to_local_arch.py --scan [目录]
    python tools/migrate_to_local_arch.py --migrate [文件]
    python tools/migrate_to_local_arch.py --validate [目录]
"""

import sys
import argparse
import re
import ast
import os
import json
from pathlib import Path
from typing import Dict, List, Set, Tuple, Optional
import logging

# 添加项目根目录到Python路径
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))


class NetworkDependencyScanner:
    """网络依赖扫描器"""
    
    def __init__(self):
        # 网络相关模式
        self.http_patterns = [
            r'import\s+requests',
            r'from\s+requests\s+import',
            r'requests\.(get|post|put|delete|patch)',
            r'urllib\.request',
            r'http\.client',
            r'httpx\.',
            r'aiohttp\.'
        ]
        
        self.api_patterns = [
            r'/api/v\d+/',
            r'localhost:\d+',
            r'127\.0\.0\.1:\d+',
            r'http://.*',
            r'https://.*',
            r'\.json\(\)',
            r'Content-Type.*application/json'
        ]
        
        self.gateway_patterns = [
            r'class.*Gateway',
            r'HTTPGateway',
            r'APIGateway',
            r'NetworkGateway',
            r'\.handle_request',
            r'\.route\('
        ]
        
        # 可替换的函数映射
        self.function_mapping = {
            # 图像处理
            'process_image': 'dispatcher.process_image_local',
            'image_compress': 'dispatcher.process_image_local', 
            'image_resize': 'dispatcher.process_image_local',
            
            # AI推理
            'ai_inference': 'dispatcher.ai_inference_local',
            'model_predict': 'dispatcher.ai_inference_local',
            'predict_params': 'dispatcher.ai_inference_local',
            
            # 特征提取  
            'extract_features': 'dispatcher.extract_features_local',
            'get_image_features': 'dispatcher.extract_features_local',
            
            # 数学运算
            'matrix_multiply': 'dispatcher.math_operation_local',
            'vector_operation': 'dispatcher.math_operation_local',
        }
    
    def scan_file(self, file_path: Path) -> Dict[str, List[Dict[str, Any]]]:
        """扫描单个文件的网络依赖"""
        results = {
            'http_calls': [],
            'api_endpoints': [],
            'gateway_usage': [],
            'imports': []
        }
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                
            lines = content.split('\n')
            
            # 扫描HTTP调用
            for i, line in enumerate(lines, 1):
                # HTTP相关导入
                for pattern in self.http_patterns:
                    if re.search(pattern, line):
                        results['imports'].append({
                            'line': i,
                            'content': line.strip(),
                            'pattern': pattern,
                            'type': 'http_import'
                        })
                
                # API调用
                for pattern in self.api_patterns:
                    if re.search(pattern, line):
                        results['api_endpoints'].append({
                            'line': i,
                            'content': line.strip(),
                            'pattern': pattern,
                            'type': 'api_call'
                        })
                
                # 网关使用
                for pattern in self.gateway_patterns:
                    if re.search(pattern, line):
                        results['gateway_usage'].append({
                            'line': i,
                            'content': line.strip(),
                            'pattern': pattern,
                            'type': 'gateway'
                        })
                
                # HTTP方法调用
                http_methods = r'(requests\.(get|post|put|delete|patch)|urllib\.request|httpx\.(get|post))'
                if re.search(http_methods, line):
                    results['http_calls'].append({
                        'line': i,
                        'content': line.strip(),
                        'method': re.search(http_methods, line).group(),
                        'type': 'http_call'
                    })
        
        except Exception as e:
            logging.error(f"扫描文件失败 {file_path}: {e}")
        
        return results
    
    def scan_directory(self, directory: Path) -> Dict[str, Dict[str, List[Dict[str, Any]]]]:
        """扫描目录的网络依赖"""
        all_results = {}
        
        for py_file in directory.glob('**/*.py'):
            if py_file.is_file():
                results = self.scan_file(py_file)
                
                # 只记录有问题的文件
                if any(results.values()):
                    all_results[str(py_file)] = results
        
        return all_results


class LocalArchMigrator:
    """本地架构迁移器"""
    
    def __init__(self):
        self.scanner = NetworkDependencyScanner()
        
        # 迁移模板
        self.migration_templates = {
            'local_imports': '''
# 本地化架构导入
from core.python.local import CallDispatcher, MemoryManager, DirectCall
from core.python.local.call_dispatcher import get_global_dispatcher
from core.python.local.memory_manager import get_global_memory_manager

# 获取全局实例
dispatcher = get_global_dispatcher()
memory_manager = get_global_memory_manager()
''',
            
            'http_to_local': '''
# 替换HTTP调用为本地函数调用
# 原来: requests.post('/api/v1/process_image', data=image_data)
# 现在: 
result = dispatcher.process_image_local(image_data, operation)
if result.success:
    processed_data = result.result
else:
    print(f"处理失败: {result.error}")
''',
            
            'memory_storage': '''
# 大数据使用内存管理器
# 原来: 通过HTTP传输大数组
# 现在:
block_id = memory_manager.store_data(large_numpy_array, zero_copy=True)
result = dispatcher.ai_inference_local(block_id, config)
processed_array = memory_manager.get_data(block_id, zero_copy=True)
'''
        }
    
    def generate_migration_plan(self, scan_results: Dict[str, Dict[str, List[Dict[str, Any]]]]) -> Dict[str, Any]:
        """生成迁移计划"""
        migration_plan = {
            'total_files': len(scan_results),
            'total_issues': 0,
            'files': {},
            'summary': {
                'http_imports_to_remove': 0,
                'http_calls_to_replace': 0,
                'api_endpoints_to_migrate': 0,
                'gateways_to_remove': 0
            }
        }
        
        for file_path, results in scan_results.items():
            file_plan = {
                'issues': [],
                'replacements': [],
                'priority': 'medium'
            }
            
            # 分析HTTP导入
            for import_issue in results.get('imports', []):
                file_plan['issues'].append({
                    'type': 'remove_import',
                    'line': import_issue['line'],
                    'description': f"移除HTTP导入: {import_issue['content']}",
                    'action': 'replace_with_local_imports'
                })
                migration_plan['summary']['http_imports_to_remove'] += 1
            
            # 分析HTTP调用
            for http_call in results.get('http_calls', []):
                file_plan['issues'].append({
                    'type': 'replace_http_call',
                    'line': http_call['line'],
                    'description': f"替换HTTP调用: {http_call['content']}",
                    'action': 'convert_to_local_call'
                })
                migration_plan['summary']['http_calls_to_replace'] += 1
            
            # 分析API端点
            for api_endpoint in results.get('api_endpoints', []):
                file_plan['issues'].append({
                    'type': 'migrate_api',
                    'line': api_endpoint['line'],
                    'description': f"迁移API端点: {api_endpoint['content']}",
                    'action': 'replace_with_function_call'
                })
                migration_plan['summary']['api_endpoints_to_migrate'] += 1
            
            # 分析网关使用
            for gateway in results.get('gateway_usage', []):
                file_plan['issues'].append({
                    'type': 'remove_gateway',
                    'line': gateway['line'],
                    'description': f"移除网关概念: {gateway['content']}",
                    'action': 'replace_with_direct_calls'
                })
                migration_plan['summary']['gateways_to_remove'] += 1
            
            # 计算优先级
            total_issues = len(file_plan['issues'])
            if total_issues > 10:
                file_plan['priority'] = 'high'
            elif total_issues > 5:
                file_plan['priority'] = 'medium'
            else:
                file_plan['priority'] = 'low'
            
            migration_plan['total_issues'] += total_issues
            migration_plan['files'][file_path] = file_plan
        
        return migration_plan
    
    def apply_migration(self, file_path: Path, migration_plan: Dict[str, Any]) -> bool:
        """应用迁移到文件"""
        try:
            # 读取原始文件
            with open(file_path, 'r', encoding='utf-8') as f:
                original_content = f.read()
            
            modified_content = original_content
            file_plan = migration_plan['files'].get(str(file_path), {})
            
            # 添加本地化导入
            if file_plan.get('issues'):
                modified_content = self._add_local_imports(modified_content)
            
            # 替换HTTP调用
            modified_content = self._replace_http_calls(modified_content)
            
            # 移除网络相关导入
            modified_content = self._remove_network_imports(modified_content)
            
            # 生成备份
            backup_path = file_path.with_suffix(file_path.suffix + '.backup')
            with open(backup_path, 'w', encoding='utf-8') as f:
                f.write(original_content)
            
            # 写入修改后的文件
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(modified_content)
            
            logging.info(f"迁移完成: {file_path}")
            logging.info(f"备份保存: {backup_path}")
            return True
            
        except Exception as e:
            logging.error(f"迁移失败 {file_path}: {e}")
            return False
    
    def _add_local_imports(self, content: str) -> str:
        """添加本地化导入"""
        lines = content.split('\n')
        
        # 寻找导入位置
        import_insert_line = 0
        for i, line in enumerate(lines):
            if line.strip().startswith(('import ', 'from ')):
                import_insert_line = i + 1
        
        # 插入本地化导入
        local_imports = [
            "# 本地化架构导入",
            "from core.python.local.call_dispatcher import get_global_dispatcher",
            "from core.python.local.memory_manager import get_global_memory_manager",
            "",
            "# 获取全局实例", 
            "dispatcher = get_global_dispatcher()",
            "memory_manager = get_global_memory_manager()",
            ""
        ]
        
        lines[import_insert_line:import_insert_line] = local_imports
        return '\n'.join(lines)
    
    def _replace_http_calls(self, content: str) -> str:
        """替换HTTP调用"""
        # 替换常见的HTTP调用模式
        replacements = [
            # requests.post API调用
            (r'requests\.post\([\'\"]/api/v\d+/process_image[\'\"]\s*,.*?\)',
             'dispatcher.process_image_local(image_data, operation)'),
            
            (r'requests\.post\([\'\"]/api/v\d+/ai_inference[\'\"]\s*,.*?\)',
             'dispatcher.ai_inference_local(features, config)'),
            
            (r'requests\.post\([\'\"]/api/v\d+/extract_features[\'\"]\s*,.*?\)',
             'dispatcher.extract_features_local(image_data)'),
            
            # requests.get API调用
            (r'requests\.get\([\'\"]/api/v\d+/.*?[\'\"]\s*\)',
             'dispatcher.call(DirectCall(function_name, args, {}))'),
            
            # 响应处理
            (r'\.json\(\)',
             '.result'),
            
            (r'response\.status_code\s*==\s*200',
             'result.success'),
        ]
        
        modified_content = content
        for pattern, replacement in replacements:
            modified_content = re.sub(pattern, replacement, modified_content, flags=re.MULTILINE | re.DOTALL)
        
        return modified_content
    
    def _remove_network_imports(self, content: str) -> str:
        """移除网络相关导入"""
        lines = content.split('\n')
        filtered_lines = []
        
        for line in lines:
            # 跳过网络相关导入
            if any(pattern in line for pattern in [
                'import requests',
                'from requests import',
                'import urllib',
                'from urllib import',
                'import httpx',
                'from httpx import',
                'import aiohttp',
                'from aiohttp import'
            ]):
                continue
            
            filtered_lines.append(line)
        
        return '\n'.join(filtered_lines)


def main():
    """主函数"""
    parser = argparse.ArgumentParser(
        description="网络架构到本地架构迁移工具",
        formatter_class=argparse.RawDescriptionHelpFormatter
    )
    
    parser.add_argument(
        '--scan', '-s',
        metavar='DIRECTORY',
        help='扫描目录中的网络依赖'
    )
    
    parser.add_argument(
        '--migrate', '-m', 
        metavar='FILE',
        help='迁移指定文件到本地架构'
    )
    
    parser.add_argument(
        '--migrate-all', '-a',
        metavar='DIRECTORY', 
        help='迁移目录中的所有文件'
    )
    
    parser.add_argument(
        '--plan', '-p',
        metavar='DIRECTORY',
        help='生成迁移计划'
    )
    
    parser.add_argument(
        '--verbose', '-v',
        action='store_true',
        help='详细输出'
    )
    
    args = parser.parse_args()
    
    # 设置日志
    logging.basicConfig(
        level=logging.DEBUG if args.verbose else logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s'
    )
    
    scanner = NetworkDependencyScanner()
    migrator = LocalArchMigrator()
    
    try:
        if args.scan:
            # 扫描网络依赖
            scan_path = Path(args.scan)
            if not scan_path.exists():
                print(f"❌ 目录不存在: {scan_path}")
                return 1
            
            print(f"🔍 扫描网络依赖: {scan_path}")
            results = scanner.scan_directory(scan_path)
            
            print(f"\n📊 扫描结果:")
            print(f"检查文件数: {len(list(scan_path.glob('**/*.py')))}")
            print(f"有问题文件: {len(results)}")
            
            for file_path, file_results in results.items():
                print(f"\n📁 {file_path}:")
                for category, issues in file_results.items():
                    if issues:
                        print(f"  {category}: {len(issues)}个问题")
                        if args.verbose:
                            for issue in issues[:3]:  # 只显示前3个
                                print(f"    第{issue['line']}行: {issue['content']}")
        
        elif args.plan:
            # 生成迁移计划
            scan_path = Path(args.plan)
            if not scan_path.exists():
                print(f"❌ 目录不存在: {scan_path}")
                return 1
            
            print(f"📋 生成迁移计划: {scan_path}")
            scan_results = scanner.scan_directory(scan_path)
            migration_plan = migrator.generate_migration_plan(scan_results)
            
            print(f"\n🎯 迁移计划摘要:")
            print(f"总文件数: {migration_plan['total_files']}")
            print(f"总问题数: {migration_plan['total_issues']}")
            print(f"HTTP导入移除: {migration_plan['summary']['http_imports_to_remove']}")
            print(f"HTTP调用替换: {migration_plan['summary']['http_calls_to_replace']}")
            print(f"API端点迁移: {migration_plan['summary']['api_endpoints_to_migrate']}")
            print(f"网关移除: {migration_plan['summary']['gateways_to_remove']}")
            
            # 按优先级分组显示
            high_priority_files = [f for f, plan in migration_plan['files'].items() 
                                 if plan['priority'] == 'high']
            medium_priority_files = [f for f, plan in migration_plan['files'].items() 
                                   if plan['priority'] == 'medium']
            
            if high_priority_files:
                print(f"\n🔥 高优先级文件 ({len(high_priority_files)}个):")
                for file_path in high_priority_files[:5]:  # 显示前5个
                    issues_count = len(migration_plan['files'][file_path]['issues'])
                    print(f"  📁 {file_path} - {issues_count}个问题")
            
            if medium_priority_files:
                print(f"\n⚠️ 中优先级文件 ({len(medium_priority_files)}个):")
                for file_path in medium_priority_files[:3]:  # 显示前3个
                    issues_count = len(migration_plan['files'][file_path]['issues'])
                    print(f"  📁 {file_path} - {issues_count}个问题")
        
        elif args.migrate:
            # 迁移单个文件
            file_path = Path(args.migrate)
            if not file_path.exists():
                print(f"❌ 文件不存在: {file_path}")
                return 1
            
            print(f"🔧 迁移文件: {file_path}")
            
            # 先扫描
            scan_results = {str(file_path): scanner.scan_file(file_path)}
            migration_plan = migrator.generate_migration_plan(scan_results)
            
            # 应用迁移
            if migrator.apply_migration(file_path, migration_plan):
                print(f"✅ 迁移成功: {file_path}")
                print(f"💾 备份文件: {file_path}.backup")
            else:
                print(f"❌ 迁移失败: {file_path}")
                return 1
        
        elif args.migrate_all:
            # 批量迁移
            scan_path = Path(args.migrate_all)
            if not scan_path.exists():
                print(f"❌ 目录不存在: {scan_path}")
                return 1
            
            print(f"🚀 批量迁移目录: {scan_path}")
            
            # 扫描和生成计划
            scan_results = scanner.scan_directory(scan_path)
            migration_plan = migrator.generate_migration_plan(scan_results)
            
            print(f"📊 发现{migration_plan['total_files']}个文件需要迁移")
            
            # 确认迁移
            if migration_plan['total_issues'] > 0:
                confirm = input(f"确定要迁移 {migration_plan['total_issues']} 个问题吗? [y/N]: ")
                if confirm.lower() != 'y':
                    print("迁移取消")
                    return 0
                
                # 执行迁移
                success_count = 0
                for file_path in migration_plan['files'].keys():
                    file_path_obj = Path(file_path)
                    if migrator.apply_migration(file_path_obj, migration_plan):
                        success_count += 1
                        print(f"✅ 迁移完成: {file_path}")
                    else:
                        print(f"❌ 迁移失败: {file_path}")
                
                print(f"\n🎉 批量迁移完成: {success_count}/{len(migration_plan['files'])} 个文件")
            else:
                print("📝 没有发现需要迁移的问题")
        
        else:
            parser.print_help()
            return 1
        
        return 0
        
    except Exception as e:
        logging.error(f"执行失败: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
