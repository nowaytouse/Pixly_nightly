"""快速压缩测试辅助函数"""
import io
from PIL import Image


def quick_compression_test(img, target_format, original_size):
    """快速压缩测试：转换缩略图估算压缩率
    
    避免AI推荐后实际转换时发现增大的问题
    
    Args:
        img: PIL Image对象
        target_format: 目标格式 (webp/avif/jxl)
        original_size: 原始文件大小(bytes)
    
    Returns:
        bool: True表示可以推荐，False表示可能增大
    """
    try:
        # 创建缩略图（最大256x256）
        thumb = img.copy()
        thumb.thumbnail((256, 256), Image.LANCZOS)
        
        # 快速转换测试
        buffer = io.BytesIO()
        if target_format == 'webp':
            thumb.save(buffer, 'WEBP', lossless=True, quality=90)
        elif target_format == 'avif':
            thumb.save(buffer, 'AVIF', quality=90)
        elif target_format == 'jxl':
            # JXL需要外部工具，跳过测试
            return True
        else:
            return True
        
        thumb_compressed = buffer.tell()
        
        # 估算缩放比例
        scale = (img.width * img.height) / (thumb.width * thumb.height)
        estimated_size = thumb_compressed * scale
        
        # 如果估算增大超过10%，返回False
        if estimated_size > original_size * 1.1:
            print(f"⚠️  快速测试：{target_format}可能增大文件 (估算{estimated_size/1024:.1f}KB > 原始{original_size/1024:.1f}KB)")
            return False
        
        return True
        
    except Exception as e:
        print(f"⚠️  快速压缩测试失败: {e}")
        return True  # 测试失败时允许推荐
