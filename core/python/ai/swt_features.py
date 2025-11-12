"""
SWT小波变换特征提取器
基于废弃Go代码 @deprecated/go_ai_service_2025_11_11/ai 2/features/swt.go 重新实现

功能:
- 高级图像特征分析（边缘强度、纹理复杂度、噪声等级）
- Sobel算子边缘检测算法实现
- 局部标准差纹理分析
- 频域能量分布计算（高/中/低频）
- 熵值计算和平滑区域比例分析
- 为AI预测提供高级特征输入

EX-009实现: 从Go废弃代码价值提取
"""

import numpy as np
import cv2
from PIL import Image
from typing import Dict, Tuple, Optional
import logging
from pathlib import Path

class ImageFeatures:
    """图像特征数据结构"""
    
    def __init__(self):
        self.edge_strength: float = 0.0
        self.texture_complexity: float = 0.0
        self.noise_level: float = 0.0
        self.detail_level: float = 0.0
        self.high_freq_energy: float = 0.0
        self.mid_freq_energy: float = 0.0
        self.low_freq_energy: float = 0.0
        self.entropy_score: float = 0.0
        self.smooth_region_ratio: float = 0.0

    def to_dict(self) -> Dict[str, float]:
        """转换为字典格式"""
        return {
            'edge_strength': self.edge_strength,
            'texture_complexity': self.texture_complexity,
            'noise_level': self.noise_level,
            'detail_level': self.detail_level,
            'high_freq_energy': self.high_freq_energy,
            'mid_freq_energy': self.mid_freq_energy,
            'low_freq_energy': self.low_freq_energy,
            'entropy_score': self.entropy_score,
            'smooth_region_ratio': self.smooth_region_ratio
        }

class SWTExtractor:
    """SWT小波变换特征提取器"""
    
    def __init__(self, levels: int = 3, debug: bool = False):
        """
        初始化SWT特征提取器
        
        Args:
            levels: 分解层数，默认3层
            debug: 是否开启调试模式
        """
        self.levels = max(1, levels)
        self.debug = debug
        
        if self.debug:
            logging.basicConfig(level=logging.DEBUG)
            self.logger = logging.getLogger(__name__)

    def extract(self, image_path: str) -> ImageFeatures:
        """
        提取SWT特征
        
        Args:
            image_path: 图像文件路径
            
        Returns:
            ImageFeatures: 提取的图像特征
            
        Raises:
            FileNotFoundError: 图像文件不存在
            ValueError: 图像格式不支持
        """
        # 验证文件存在
        if not Path(image_path).exists():
            raise FileNotFoundError(f"图像文件不存在: {image_path}")
        
        try:
            # 加载并转换为灰度图
            image = Image.open(image_path)
            gray_array = self._to_grayscale(image)
            
            # 计算各种特征
            features = ImageFeatures()
            features.edge_strength = self._calculate_edge_strength(gray_array)
            features.texture_complexity = self._calculate_texture_complexity(gray_array)
            features.noise_level = self._calculate_noise_level(gray_array)
            features.detail_level = self._calculate_detail_level(gray_array)
            
            # 计算频域能量
            high_freq, mid_freq, low_freq = self._calculate_frequency_energy(gray_array)
            features.high_freq_energy = high_freq
            features.mid_freq_energy = mid_freq
            features.low_freq_energy = low_freq
            
            features.entropy_score = self._calculate_entropy(gray_array)
            features.smooth_region_ratio = self._calculate_smooth_region_ratio(gray_array)
            
            if self.debug:
                self.logger.debug(
                    f"[SWTExtractor] 边缘强度={features.edge_strength:.2f}, "
                    f"纹理复杂度={features.texture_complexity:.2f}, "
                    f"噪声={features.noise_level:.2f}"
                )
            
            return features
            
        except Exception as e:
            raise ValueError(f"无法处理图像: {e}")

    def _to_grayscale(self, image: Image.Image) -> np.ndarray:
        """转换为灰度图像数组"""
        if image.mode != 'L':
            image = image.convert('L')
        return np.array(image)

    def _calculate_edge_strength(self, gray_img: np.ndarray) -> float:
        """
        计算边缘强度
        使用Sobel算子近似
        """
        height, width = gray_img.shape
        
        # Sobel核
        sobel_x = np.array([[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]], dtype=np.float32)
        sobel_y = np.array([[-1, -2, -1], [0, 0, 0], [1, 2, 1]], dtype=np.float32)
        
        # 使用OpenCV的滤波器会更快，但为了与Go版本保持一致，手动实现
        total_edge = 0.0
        count = 0
        
        for y in range(1, height - 1):
            for x in range(1, width - 1):
                gx = 0.0
                gy = 0.0
                
                # 应用Sobel算子
                for dy in range(-1, 2):
                    for dx in range(-1, 2):
                        pixel = float(gray_img[y + dy, x + dx])
                        gx += pixel * sobel_x[dy + 1, dx + 1]
                        gy += pixel * sobel_y[dy + 1, dx + 1]
                
                # 计算梯度幅值
                magnitude = np.sqrt(gx * gx + gy * gy)
                total_edge += magnitude
                count += 1
        
        if count == 0:
            return 0.0
        
        # 归一化到0-100
        return (total_edge / count) / 255.0 * 100.0

    def _calculate_texture_complexity(self, gray_img: np.ndarray) -> float:
        """
        计算纹理复杂度
        使用局部标准差
        """
        height, width = gray_img.shape
        window_size = 5
        half_window = window_size // 2
        
        total_std = 0.0
        count = 0
        
        for y in range(half_window, height - half_window):
            for x in range(half_window, width - half_window):
                # 提取局部窗口
                window = gray_img[y-half_window:y+half_window+1, x-half_window:x+half_window+1]
                std_dev = np.std(window.astype(np.float32))
                total_std += std_dev
                count += 1
        
        if count == 0:
            return 0.0
        
        # 归一化到0-100
        return (total_std / count) / 255.0 * 100.0

    def _calculate_noise_level(self, gray_img: np.ndarray) -> float:
        """
        计算噪声等级
        使用拉普拉斯算子的方差
        """
        # 拉普拉斯核
        laplacian_kernel = np.array([[0, -1, 0], [-1, 4, -1], [0, -1, 0]], dtype=np.float32)
        
        # 应用拉普拉斯算子
        filtered = cv2.filter2D(gray_img.astype(np.float32), -1, laplacian_kernel)
        
        # 计算方差作为噪声估计
        noise_variance = np.var(filtered)
        
        # 归一化到0-100
        return min(100.0, noise_variance / 10000.0 * 100.0)

    def _calculate_detail_level(self, gray_img: np.ndarray) -> float:
        """
        计算细节等级
        基于高频信息的能量
        """
        # 使用高斯模糊和原图的差值来估计细节
        blurred = cv2.GaussianBlur(gray_img.astype(np.float32), (15, 15), 5.0)
        detail = np.abs(gray_img.astype(np.float32) - blurred)
        
        # 计算细节的平均能量
        detail_energy = np.mean(detail)
        
        # 归一化到0-100
        return min(100.0, detail_energy / 255.0 * 100.0)

    def _calculate_frequency_energy(self, gray_img: np.ndarray) -> Tuple[float, float, float]:
        """
        计算频域能量分布
        
        Returns:
            Tuple[high_freq_energy, mid_freq_energy, low_freq_energy]
        """
        # 进行2D FFT
        f_transform = np.fft.fft2(gray_img.astype(np.float32))
        f_shift = np.fft.fftshift(f_transform)
        magnitude_spectrum = np.abs(f_shift)
        
        rows, cols = gray_img.shape
        crow, ccol = rows // 2, cols // 2
        
        # 创建频域掩码
        y, x = np.ogrid[:rows, :cols]
        center = np.sqrt((x - ccol) ** 2 + (y - crow) ** 2)
        
        # 定义频域范围
        low_freq_mask = center <= min(rows, cols) * 0.1
        high_freq_mask = center >= min(rows, cols) * 0.3
        mid_freq_mask = ~(low_freq_mask | high_freq_mask)
        
        # 计算各频域的能量
        low_freq_energy = np.sum(magnitude_spectrum[low_freq_mask])
        mid_freq_energy = np.sum(magnitude_spectrum[mid_freq_mask])
        high_freq_energy = np.sum(magnitude_spectrum[high_freq_mask])
        
        # 归一化
        total_energy = low_freq_energy + mid_freq_energy + high_freq_energy
        if total_energy > 0:
            low_freq_energy = (low_freq_energy / total_energy) * 100.0
            mid_freq_energy = (mid_freq_energy / total_energy) * 100.0
            high_freq_energy = (high_freq_energy / total_energy) * 100.0
        
        return high_freq_energy, mid_freq_energy, low_freq_energy

    def _calculate_entropy(self, gray_img: np.ndarray) -> float:
        """
        计算图像熵
        """
        # 计算直方图
        histogram, _ = np.histogram(gray_img, bins=256, range=(0, 256))
        
        # 归一化概率
        histogram = histogram.astype(np.float32)
        histogram /= histogram.sum()
        
        # 计算熵
        entropy = 0.0
        for p in histogram:
            if p > 0:
                entropy -= p * np.log2(p)
        
        # 归一化到0-100
        return (entropy / 8.0) * 100.0  # 8是8位图像的最大熵

    def _calculate_smooth_region_ratio(self, gray_img: np.ndarray) -> float:
        """
        计算平滑区域比例
        """
        # 计算局部方差
        kernel_size = 7
        kernel = np.ones((kernel_size, kernel_size), np.float32) / (kernel_size * kernel_size)
        
        # 计算局部均值和方差
        mean_img = cv2.filter2D(gray_img.astype(np.float32), -1, kernel)
        sqr_img = cv2.filter2D((gray_img.astype(np.float32)) ** 2, -1, kernel)
        var_img = sqr_img - mean_img ** 2
        
        # 定义平滑区域阈值
        smooth_threshold = 100.0  # 方差阈值
        smooth_mask = var_img < smooth_threshold
        
        # 计算平滑区域比例
        smooth_ratio = np.sum(smooth_mask) / (gray_img.shape[0] * gray_img.shape[1])
        
        return smooth_ratio * 100.0


def extract_swt_features(image_path: str, debug: bool = False) -> Dict[str, float]:
    """
    便捷函数：提取SWT特征
    
    Args:
        image_path: 图像路径
        debug: 调试模式
        
    Returns:
        Dict: 特征字典
    """
    extractor = SWTExtractor(debug=debug)
    features = extractor.extract(image_path)
    return features.to_dict()


if __name__ == "__main__":
    # 测试代码
    import sys
    
    if len(sys.argv) > 1:
        image_path = sys.argv[1]
        features = extract_swt_features(image_path, debug=True)
        
        print("\n=== SWT特征提取结果 ===")
        for key, value in features.items():
            print(f"{key}: {value:.2f}")
    else:
        print("Usage: python swt_features.py <image_path>")
