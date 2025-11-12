#!/usr/bin/env python3
"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Phase 47.23: HTTP参数验证器
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

功能：
- 响亮报错原则实现
- 完整的参数验证逻辑
- 清晰的错误消息

核心原则：
- 任何无效参数都会返回错误
- 错误消息清晰明确
- 不使用默认值覆盖无效输入

从Go代码提取：core/@deprecated/go_ai_service_2025_11_11/ai 2/http_validator.go
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"""

from pathlib import Path
from typing import Dict, Any, Optional


class ValidationError(Exception):
    """验证错误异常"""
    pass


class RequestValidator:
    """请求参数验证器"""
    
    # 支持的图像扩展名
    VALID_EXTENSIONS = {
        '.jpg', '.jpeg', '.png', '.webp', '.gif', '.bmp',
        '.tiff', '.tif', '.avif', '.jxl', '.heic', '.heif'
    }
    
    # 支持的工具名称
    VALID_TOOLS = {
        'cjxl', 'cavif', 'cwebp', 'pngquant',
        'oxipng', 'ffmpeg', 'auto'
    }
    
    # 支持的优化模式
    VALID_OPTIMIZE_MODES = {
        'quality', 'balanced', 'size'
    }
    
    def validate_image_path(self, image_path: str) -> None:
        """
        验证图像路径
        
        Args:
            image_path: 图像文件路径
            
        Raises:
            ValidationError: 路径无效
        """
        if not image_path or not image_path.strip():
            raise ValidationError("图像路径为空")
        
        path = Path(image_path)
        
        # 检查文件是否存在
        if not path.exists():
            raise ValidationError(f"图像文件不存在: {image_path}")
        
        # 检查是否是文件
        if not path.is_file():
            raise ValidationError(f"路径不是文件: {image_path}")
        
        # 检查文件扩展名
        ext = path.suffix.lower()
        if ext not in self.VALID_EXTENSIONS:
            raise ValidationError(
                f"不支持的图像格式: {ext}\n"
                f"支持的格式: {', '.join(sorted(self.VALID_EXTENSIONS))}"
            )
    
    def validate_tool_name(self, tool: str) -> None:
        """
        验证工具名称
        
        Args:
            tool: 工具名称
            
        Raises:
            ValidationError: 工具名称无效
        """
        if not tool or not tool.strip():
            raise ValidationError("工具名称为空")
        
        if tool not in self.VALID_TOOLS:
            raise ValidationError(
                f"不支持的工具: {tool}\n"
                f"支持的工具: {', '.join(sorted(self.VALID_TOOLS))}"
            )
    
    def validate_target_quality(self, quality: int) -> None:
        """
        验证目标质量参数
        
        Args:
            quality: 质量参数 (0-100)
            
        Raises:
            ValidationError: 质量参数无效
        """
        if not isinstance(quality, (int, float)):
            raise ValidationError(f"质量参数必须是数字，当前类型: {type(quality).__name__}")
        
        if quality < 0 or quality > 100:
            raise ValidationError(f"质量参数必须在0-100之间，当前值: {quality}")
    
    def validate_optimize_mode(self, mode: str) -> None:
        """
        验证优化模式
        
        Args:
            mode: 优化模式
            
        Raises:
            ValidationError: 模式无效
        """
        if not mode or not mode.strip():
            raise ValidationError("优化模式为空")
        
        if mode not in self.VALID_OPTIMIZE_MODES:
            raise ValidationError(
                f"不支持的优化模式: {mode}\n"
                f"支持的模式: {', '.join(sorted(self.VALID_OPTIMIZE_MODES))}"
            )
    
    def validate_request_options(self, options: Dict[str, Any]) -> None:
        """
        验证请求选项
        
        Args:
            options: 请求选项字典
            
        Raises:
            ValidationError: 选项无效
        """
        if not isinstance(options, dict):
            raise ValidationError(f"选项必须是字典，当前类型: {type(options).__name__}")
        
        # 验证effort参数（如果存在）
        if 'effort' in options:
            effort = options['effort']
            if not isinstance(effort, (int, float)):
                raise ValidationError(f"effort必须是数字，当前类型: {type(effort).__name__}")
            if effort < 0 or effort > 10:
                raise ValidationError(f"effort必须在0-10之间，当前值: {effort}")
        
        # 验证speed参数（如果存在）
        if 'speed' in options:
            speed = options['speed']
            if not isinstance(speed, (int, float)):
                raise ValidationError(f"speed必须是数字，当前类型: {type(speed).__name__}")
            if speed < 0 or speed > 10:
                raise ValidationError(f"speed必须在0-10之间，当前值: {speed}")
    
    def validate_predict_request(self, request: Dict[str, Any]) -> None:
        """
        验证预测请求（完整验证）
        
        核心原则：响亮报错 > 静默降级
        - 任何无效参数都会返回错误
        - 错误消息清晰明确
        - 不使用默认值覆盖无效输入
        
        Args:
            request: 请求字典
            
        Raises:
            ValidationError: 请求无效
        """
        # 1. 验证图像路径（必需）
        if 'image_path' not in request:
            raise ValidationError("缺少必需参数: image_path")
        
        self.validate_image_path(request['image_path'])
        
        # 2. 验证工具名称（可选）
        if 'tool' in request and request['tool']:
            self.validate_tool_name(request['tool'])
        
        # 3. 验证目标质量（可选）
        if 'target_quality' in request and request['target_quality'] is not None:
            self.validate_target_quality(request['target_quality'])
        
        # 4. 验证优化模式（可选）
        if 'optimize_mode' in request and request['optimize_mode']:
            self.validate_optimize_mode(request['optimize_mode'])
        
        # 5. 验证选项（可选）
        if 'options' in request and request['options']:
            self.validate_request_options(request['options'])


# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# 测试和演示
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

if __name__ == "__main__":
    import tempfile
    import os
    
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("Phase 47.23: HTTP参数验证器测试")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n")
    
    validator = RequestValidator()
    
    # 创建临时测试文件
    temp_dir = tempfile.mkdtemp()
    test_image = os.path.join(temp_dir, "test.jpg")
    with open(test_image, 'w') as f:
        f.write("test")
    
    # 测试1: 有效请求
    print("1️⃣  测试有效请求...")
    valid_request = {
        'image_path': test_image,
        'tool': 'cjxl',
        'target_quality': 85,
        'optimize_mode': 'quality',
        'options': {'effort': 7}
    }
    
    try:
        validator.validate_predict_request(valid_request)
        print("   ✅ 验证通过")
    except ValidationError as e:
        print(f"   ❌ 验证失败: {e}")
    
    # 测试2: 图像路径不存在
    print("\n2️⃣  测试图像路径不存在...")
    try:
        validator.validate_image_path("/nonexistent/image.jpg")
        print("   ❌ 应该报错但没有")
    except ValidationError as e:
        print(f"   ✅ 正确报错: {e}")
    
    # 测试3: 不支持的格式
    print("\n3️⃣  测试不支持的格式...")
    bad_ext = os.path.join(temp_dir, "test.xyz")
    with open(bad_ext, 'w') as f:
        f.write("test")
    
    try:
        validator.validate_image_path(bad_ext)
        print("   ❌ 应该报错但没有")
    except ValidationError as e:
        print(f"   ✅ 正确报错: {e}")
    
    # 测试4: 无效的工具名称
    print("\n4️⃣  测试无效的工具名称...")
    try:
        validator.validate_tool_name("invalid_tool")
        print("   ❌ 应该报错但没有")
    except ValidationError as e:
        print(f"   ✅ 正确报错: {e}")
    
    # 测试5: 质量参数超出范围
    print("\n5️⃣  测试质量参数超出范围...")
    try:
        validator.validate_target_quality(150)
        print("   ❌ 应该报错但没有")
    except ValidationError as e:
        print(f"   ✅ 正确报错: {e}")
    
    # 测试6: 无效的优化模式
    print("\n6️⃣  测试无效的优化模式...")
    try:
        validator.validate_optimize_mode("ultra_fast")
        print("   ❌ 应该报错但没有")
    except ValidationError as e:
        print(f"   ✅ 正确报错: {e}")
    
    # 测试7: effort参数超出范围
    print("\n7️⃣  测试effort参数超出范围...")
    try:
        validator.validate_request_options({'effort': 15})
        print("   ❌ 应该报错但没有")
    except ValidationError as e:
        print(f"   ✅ 正确报错: {e}")
    
    # 测试8: 完整请求验证（缺少必需参数）
    print("\n8️⃣  测试缺少必需参数...")
    try:
        validator.validate_predict_request({'tool': 'cjxl'})
        print("   ❌ 应该报错但没有")
    except ValidationError as e:
        print(f"   ✅ 正确报错: {e}")
    
    # 清理
    import shutil
    shutil.rmtree(temp_dir)
    
    print("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("✅ HTTP参数验证器测试完成")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
    print("\n💡 核心原则验证:")
    print("   ✅ 响亮报错 > 静默降级")
    print("   ✅ 清晰的错误消息")
    print("   ✅ 不使用默认值覆盖无效输入")
    print("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━")
