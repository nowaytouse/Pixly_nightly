"""
XMP元数据处理器

Phase 47.21 (EX-003): 从废弃Go代码提取
源文件: @archive/deprecated_phase47_archived/archive/standalone_tools/merge_xmp/

核心功能:
- XMP侧边车文件自动查找
- XMP元数据解析
- 元数据合并到图像文件

作者: Pixly Team
日期: 2025-11-12
"""

from pathlib import Path
from typing import Optional, Dict, Any, List
import xml.etree.ElementTree as ET
from dataclasses import dataclass


@dataclass
class XMPMetadata:
    """XMP元数据（增强版）"""
    # Dublin Core
    title: Optional[str] = None
    description: Optional[str] = None
    creator: Optional[str] = None
    keywords: List[str] = None
    subject: Optional[str] = None
    rights: Optional[str] = None
    
    # XMP Basic
    rating: Optional[int] = None
    create_date: Optional[str] = None
    modify_date: Optional[str] = None
    metadata_date: Optional[str] = None
    label: Optional[str] = None
    
    # IPTC Core
    iptc_keywords: List[str] = None
    city: Optional[str] = None
    country: Optional[str] = None
    copyright: Optional[str] = None
    credit: Optional[str] = None
    source: Optional[str] = None
    
    # EXIF
    camera_make: Optional[str] = None
    camera_model: Optional[str] = None
    lens_model: Optional[str] = None
    focal_length: Optional[str] = None
    aperture: Optional[str] = None
    shutter_speed: Optional[str] = None
    iso: Optional[int] = None
    
    raw_data: Dict[str, Any] = None
    
    def __post_init__(self):
        if self.keywords is None:
            self.keywords = []
        if self.iptc_keywords is None:
            self.iptc_keywords = []
        if self.raw_data is None:
            self.raw_data = {}


class XMPProcessor:
    """XMP元数据处理器"""
    
    # XMP命名空间（扩展）
    NAMESPACES = {
        'rdf': 'http://www.w3.org/1999/02/22-rdf-syntax-ns#',
        'dc': 'http://purl.org/dc/elements/1.1/',
        'xmp': 'http://ns.adobe.com/xap/1.0/',
        'xmpRights': 'http://ns.adobe.com/xap/1.0/rights/',
        'photoshop': 'http://ns.adobe.com/photoshop/1.0/',
        'exif': 'http://ns.adobe.com/exif/1.0/',
        'tiff': 'http://ns.adobe.com/tiff/1.0/',
        'Iptc4xmpCore': 'http://iptc.org/std/Iptc4xmpCore/1.0/xmlns/',
        'aux': 'http://ns.adobe.com/exif/1.0/aux/'
    }
    
    @staticmethod
    def find_xmp_sidecar(image_path: Path) -> Optional[Path]:
        """
        查找XMP侧边车文件
        
        支持的命名模式:
        1. image.jpg -> image.jpg.xmp
        2. image.jpg -> image.xmp
        3. image.jpg -> image.JPG.xmp (大小写变体)
        
        Args:
            image_path: 图像文件路径
            
        Returns:
            XMP文件路径，如果不存在则返回None
        """
        if not image_path.exists():
            return None
        
        # 模式1: image.jpg.xmp
        xmp_path = image_path.with_suffix(image_path.suffix + '.xmp')
        if xmp_path.exists():
            return xmp_path
        
        # 模式2: image.xmp
        xmp_path = image_path.with_suffix('.xmp')
        if xmp_path.exists():
            return xmp_path
        
        # 模式3: image.JPG.xmp (大小写变体)
        xmp_path = image_path.with_suffix(image_path.suffix.upper() + '.xmp')
        if xmp_path.exists():
            return xmp_path
        
        # 模式4: 同名但大写扩展名
        xmp_path = image_path.with_suffix('.XMP')
        if xmp_path.exists():
            return xmp_path
        
        return None
    
    @staticmethod
    def parse_xmp(xmp_path: Path) -> XMPMetadata:
        """
        解析XMP文件
        
        Args:
            xmp_path: XMP文件路径
            
        Returns:
            XMP元数据对象
        """
        metadata = XMPMetadata()
        
        try:
            # 读取XMP文件
            with open(xmp_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # 查找XMP包开始位置
            xmp_start = content.find('<x:xmpmeta')
            if xmp_start == -1:
                xmp_start = content.find('<?xpacket')
            
            if xmp_start != -1:
                # 提取XMP部分
                xmp_end = content.find('</x:xmpmeta>', xmp_start)
                if xmp_end == -1:
                    xmp_end = content.find('<?xpacket end', xmp_start)
                
                if xmp_end != -1:
                    xmp_content = content[xmp_start:xmp_end+20]  # 包含结束标签
                else:
                    xmp_content = content[xmp_start:]
            else:
                # 整个文件作为XMP
                xmp_content = content
            
            # 解析XML
            root = ET.fromstring(xmp_content)
            
            # 注册命名空间
            for prefix, uri in XMPProcessor.NAMESPACES.items():
                ET.register_namespace(prefix, uri)
            
            # 提取Dublin Core元数据
            metadata.title = XMPProcessor._extract_text(root, './/dc:title', 'dc')
            metadata.description = XMPProcessor._extract_text(root, './/dc:description', 'dc')
            metadata.creator = XMPProcessor._extract_text(root, './/dc:creator', 'dc')
            
            # 提取关键词
            keywords_elem = root.find('.//dc:subject', XMPProcessor.NAMESPACES)
            if keywords_elem is not None:
                bag = keywords_elem.find('.//rdf:Bag', XMPProcessor.NAMESPACES)
                if bag is not None:
                    metadata.keywords = [
                        li.text for li in bag.findall('.//rdf:li', XMPProcessor.NAMESPACES)
                        if li.text
                    ]
            
            # 提取XMP基本元数据
            metadata.create_date = XMPProcessor._extract_text(root, './/xmp:CreateDate', 'xmp')
            metadata.modify_date = XMPProcessor._extract_text(root, './/xmp:ModifyDate', 'xmp')
            metadata.metadata_date = XMPProcessor._extract_text(root, './/xmp:MetadataDate', 'xmp')
            metadata.label = XMPProcessor._extract_text(root, './/xmp:Label', 'xmp')
            
            # 提取评分
            rating_text = XMPProcessor._extract_text(root, './/xmp:Rating', 'xmp')
            if rating_text:
                try:
                    metadata.rating = int(rating_text)
                except ValueError:
                    pass
            
            # 提取Dublin Core扩展
            metadata.subject = XMPProcessor._extract_text(root, './/dc:subject', 'dc')
            metadata.rights = XMPProcessor._extract_text(root, './/dc:rights', 'dc')
            
            # 提取IPTC Core元数据
            metadata.city = XMPProcessor._extract_text(root, './/Iptc4xmpCore:City', 'Iptc4xmpCore')
            metadata.country = XMPProcessor._extract_text(root, './/Iptc4xmpCore:CountryName', 'Iptc4xmpCore')
            metadata.copyright = XMPProcessor._extract_text(root, './/xmpRights:UsageTerms', 'xmpRights')
            metadata.credit = XMPProcessor._extract_text(root, './/photoshop:Credit', 'photoshop')
            metadata.source = XMPProcessor._extract_text(root, './/photoshop:Source', 'photoshop')
            
            # 提取EXIF元数据
            metadata.camera_make = XMPProcessor._extract_text(root, './/tiff:Make', 'tiff')
            metadata.camera_model = XMPProcessor._extract_text(root, './/tiff:Model', 'tiff')
            metadata.lens_model = XMPProcessor._extract_text(root, './/aux:Lens', 'aux')
            metadata.focal_length = XMPProcessor._extract_text(root, './/exif:FocalLength', 'exif')
            metadata.aperture = XMPProcessor._extract_text(root, './/exif:FNumber', 'exif')
            metadata.shutter_speed = XMPProcessor._extract_text(root, './/exif:ExposureTime', 'exif')
            
            iso_text = XMPProcessor._extract_text(root, './/exif:ISOSpeedRatings', 'exif')
            if iso_text:
                try:
                    metadata.iso = int(iso_text)
                except ValueError:
                    pass
            
            # 保存原始数据
            metadata.raw_data = {
                'file_path': str(xmp_path),
                'file_size': xmp_path.stat().st_size
            }
            
        except Exception as e:
            print(f"⚠️ 解析XMP文件失败 {xmp_path}: {e}")
        
        return metadata
    
    @staticmethod
    def _extract_text(root: ET.Element, xpath: str, namespace: str) -> Optional[str]:
        """从XML提取文本内容"""
        elem = root.find(xpath, XMPProcessor.NAMESPACES)
        if elem is not None:
            # 检查rdf:Alt结构
            alt = elem.find('.//rdf:Alt/rdf:li', XMPProcessor.NAMESPACES)
            if alt is not None and alt.text:
                return alt.text.strip()
            
            # 直接文本
            if elem.text:
                return elem.text.strip()
        
        return None
    
    @staticmethod
    def merge_xmp_to_image(image_path: Path, dry_run: bool = True) -> bool:
        """
        将XMP侧边车合并到图像文件
        
        Args:
            image_path: 图像文件路径
            dry_run: 是否为试运行（不实际修改文件）
            
        Returns:
            是否成功合并
        """
        # 查找XMP文件
        xmp_path = XMPProcessor.find_xmp_sidecar(image_path)
        if not xmp_path:
            print(f"⚠️ 未找到XMP侧边车文件: {image_path}")
            return False
        
        print(f"✅ 找到XMP文件: {xmp_path.name}")
        
        # 解析XMP
        metadata = XMPProcessor.parse_xmp(xmp_path)
        
        print(f"📋 XMP元数据:")
        if metadata.title:
            print(f"   标题: {metadata.title}")
        if metadata.description:
            print(f"   描述: {metadata.description[:100]}...")
        if metadata.creator:
            print(f"   创建者: {metadata.creator}")
        if metadata.keywords:
            print(f"   关键词: {', '.join(metadata.keywords[:5])}")
        if metadata.rating:
            print(f"   评分: {'⭐' * metadata.rating}")
        if metadata.city or metadata.country:
            location = f"{metadata.city or ''}, {metadata.country or ''}".strip(', ')
            print(f"   位置: {location}")
        if metadata.camera_model:
            print(f"   相机: {metadata.camera_make or ''} {metadata.camera_model}")
        if metadata.lens_model:
            print(f"   镜头: {metadata.lens_model}")
        if metadata.focal_length or metadata.aperture or metadata.iso:
            settings = []
            if metadata.focal_length:
                settings.append(metadata.focal_length)
            if metadata.aperture:
                settings.append(f"f/{metadata.aperture}")
            if metadata.shutter_speed:
                settings.append(metadata.shutter_speed)
            if metadata.iso:
                settings.append(f"ISO{metadata.iso}")
            if settings:
                print(f"   拍摄参数: {' | '.join(settings)}")
        
        if dry_run:
            print(f"🔍 试运行模式 - 不修改文件")
            return True
        
        # 实际合并（需要exiftool或Pillow）
        try:
            # 方法1: 使用exiftool（如果可用）
            import subprocess
            
            result = subprocess.run(
                ['exiftool', '-tagsFromFile', str(xmp_path), str(image_path)],
                capture_output=True,
                text=True
            )
            
            if result.returncode == 0:
                print(f"✅ XMP元数据已合并到 {image_path.name}")
                return True
            else:
                print(f"❌ exiftool合并失败: {result.stderr}")
                return False
                
        except FileNotFoundError:
            print(f"⚠️ exiftool未安装，无法合并元数据")
            print(f"   安装方法: brew install exiftool")
            return False
    
    @staticmethod
    def batch_find_xmp_files(directory: Path, recursive: bool = True) -> List[tuple]:
        """
        批量查找有XMP侧边车的图像文件
        
        Args:
            directory: 目录路径
            recursive: 是否递归搜索
            
        Returns:
            (image_path, xmp_path) 元组列表
        """
        pairs = []
        
        # 支持的图像格式
        image_extensions = {'.jpg', '.jpeg', '.png', '.tif', '.tiff', '.cr2', '.nef', '.arw'}
        
        # 搜索模式
        pattern = '**/*' if recursive else '*'
        
        for image_path in directory.glob(pattern):
            if image_path.suffix.lower() in image_extensions:
                xmp_path = XMPProcessor.find_xmp_sidecar(image_path)
                if xmp_path:
                    pairs.append((image_path, xmp_path))
        
        return pairs
    
    @staticmethod
    def print_xmp_summary(directory: Path):
        """打印目录的XMP统计"""
        pairs = XMPProcessor.batch_find_xmp_files(directory)
        
        print("\n" + "="*60)
        print(f"📊 XMP侧边车统计: {directory}")
        print("="*60)
        print(f"找到 {len(pairs)} 个图像文件有XMP侧边车")
        
        if pairs:
            print(f"\n示例文件:")
            for image_path, xmp_path in pairs[:5]:
                print(f"  📸 {image_path.name}")
                print(f"     -> {xmp_path.name}")
        
        print("="*60 + "\n")


# 示例使用
if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1:
        # 命令行模式
        target = Path(sys.argv[1])
        
        if target.is_dir():
            # 目录统计
            XMPProcessor.print_xmp_summary(target)
        elif target.is_file():
            # 单个文件处理
            XMPProcessor.merge_xmp_to_image(target, dry_run=True)
    else:
        # 测试模式
        print("🧪 XMP处理器测试模式")
        print("\n用法:")
        print("  python xmp_processor.py <directory>  # 统计目录中的XMP文件")
        print("  python xmp_processor.py <image>      # 处理单个图像文件")
        
        # 示例
        print("\n示例: 查找当前目录的XMP文件")
        current_dir = Path(".")
        pairs = XMPProcessor.batch_find_xmp_files(current_dir, recursive=False)
        print(f"找到 {len(pairs)} 个图像文件有XMP侧边车")
