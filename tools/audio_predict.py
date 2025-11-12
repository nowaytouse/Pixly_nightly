#!/usr/bin/env python3
"""
Audio AI Prediction Module
统一策略：与图像/视频完全对齐

功能：
- 音频格式推荐（保持质量前提下优化体积）
- 智能编码器升级（AAC, Opus, FLAC）
- 统一参数命名和返回值结构
- Magika AI 文件检测（默认启用）
"""

import subprocess
import json
from pathlib import Path

try:
    from pixly_logging import log_info, log_warn, log_debug
except ImportError:
    def log_info(comp, msg, **ctx): print(f"✅ [{comp}] {msg}")
    def log_warn(comp, msg, **ctx): print(f"⚠️  [{comp}] {msg}")
    def log_debug(comp, msg, **ctx): print(f"🔍 [{comp}] {msg}")

# Magika AI检测（可选依赖）
try:
    from magika import Magika
    MAGIKA_AVAILABLE = True
except ImportError:
    MAGIKA_AVAILABLE = False


class AudioPredictor:
    """音频参数AI预测器 - 统一策略"""
    
    def __init__(self):
        """初始化音频预测器"""
        # 🆕 Magika AI检测器（默认启用）
        if MAGIKA_AVAILABLE:
            try:
                self.magika = Magika()
                log_info("AudioAI", "Magika AI detector initialized")
            except Exception as e:
                log_warn("AudioAI", f"Magika init failed: {e}")
                self.magika = None
        else:
            self.magika = None
            log_warn("AudioAI", "Magika not available, detection disabled")
    
    def predict_audio(self, audio_path, optimize_mode='balanced', options=None):
        """
        音频参数AI预测（统一策略）
        
        Args:
            audio_path: 音频文件路径
            optimize_mode: 优化模式 (balanced/quality) - 仅quality和balanced，无size模式
            options: 可选参数 {
                - expected_codec: 期望输出编码器 (aac, opus, flac, mp3)
                  同图像/视频的expected_format/expected_codec
                - enable_format_recommendation: 启用AI编码器推荐（默认True）
                - enable_preprocessing: 启用预处理建议（默认True）
                - aggressive_mode: 激进编码器模式（默认False）
                - enable_magika: 启用Magika AI检测（默认True）
            }
        
        Returns:
            {
                'encoder': str,              # 推荐编码器
                'bitrate': int,              # 推荐比特率 (kbps)
                'sample_rate': int,          # 采样率
                'channels': int,             # 声道数
                'reasoning': str,            # AI推理原因
                'confidence': float,         # 置信度
                'source_codec': str,         # 源编码器
                'recommended_codec': str,    # AI推荐编码器
                'codec_reason': str,         # 编码器推荐原因
                'preprocessing_steps': [],   # 预处理建议
                'magika_detection': {},      # Magika AI检测结果
                'metadata': {                # 音频元数据
                    'duration': float,
                    'bitrate': int,
                    'sample_rate': int,
                    'channels': int,
                    'codec': str
                }
            }
        """
        options = options or {}
        expected_codec = options.get('expected_codec', None)
        enable_format_recommendation = options.get('enable_format_recommendation', True)
        enable_preprocessing = options.get('enable_preprocessing', True)
        aggressive_mode = options.get('aggressive_mode', False)
        enable_magika = options.get('enable_magika', True)  # 🆕 默认启用
        
        log_info('AudioAI', f"Analyzing audio: {audio_path}")
        
        # 🆕 Magika AI文件检测（增强安全性）
        magika_result = {}
        if enable_magika and self.magika:
            try:
                detection = self.magika.identify_path(Path(audio_path))
                magika_result = {
                    'detected_type': detection.output.ct_label,
                    'confidence': detection.output.score,
                    'mime_type': detection.output.mime_type,
                    'is_audio': detection.output.group == 'audio'
                }
                log_info('AudioAI', f"Magika: {detection.output.ct_label} ({detection.output.score:.2%})")
                
                # 安全验证
                if not magika_result['is_audio']:
                    log_warn('AudioAI', f"File is not audio: {magika_result['detected_type']}")
            except Exception as e:
                log_warn('AudioAI', f"Magika detection failed: {e}")
        
        # 提取音频元数据
        try:
            cmd = [
                'ffprobe', '-v', 'error',
                '-show_entries', 'format=duration,bit_rate,format_name',
                '-show_entries', 'stream=codec_name,sample_rate,channels,codec_type',
                '-of', 'json',
                audio_path
            ]
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
            metadata = json.loads(result.stdout)
            
            # 提取音频流信息
            audio_stream = None
            for stream in metadata.get('streams', []):
                if stream.get('codec_type') == 'audio':
                    audio_stream = stream
                    break
            
            if not audio_stream:
                raise ValueError("No audio stream found")
            
            source_codec = audio_stream.get('codec_name', 'unknown')
            sample_rate = int(audio_stream.get('sample_rate', 48000))
            channels = int(audio_stream.get('channels', 2))
            
            format_info = metadata.get('format', {})
            duration = float(format_info.get('duration', 0))
            file_size = int(format_info.get('size', 0))
            bitrate = int(format_info.get('bit_rate', 0)) // 1000  # to kbps
            
            log_info('AudioAI', f"Metadata: {source_codec}, {sample_rate}Hz, {channels}ch, {bitrate}kbps, {duration:.2f}s, {file_size}bytes")
            
            # 🎯 极小音频保护（<10KB 或 <1秒）：转换开销可能大于压缩收益
            if file_size > 0 and file_size < 10240:  # <10KB
                log_warn('AudioAI', f"极小音频文件 ({file_size}bytes)，建议保持原格式")
                return {
                    'success': False,
                    'skip_reason': 'tiny_file',
                    'message': f'File too small ({file_size}bytes), conversion overhead may exceed compression benefit',
                    'recommended_action': 'keep_original'
                }
            if duration > 0 and duration < 1.0:  # <1秒
                log_warn('AudioAI', f"极短音频 ({duration:.2f}s)，建议保持原格式")
                return {
                    'success': False,
                    'skip_reason': 'short_duration',
                    'message': f'Audio too short ({duration:.2f}s), conversion overhead may exceed compression benefit',
                    'recommended_action': 'keep_original'
                }
            
        except Exception as e:
            log_warn('AudioAI', f"Metadata extraction failed: {e}")
            return {
                'error': str(e),
                'error_code': 'AUDIO_METADATA_FAILED'
            }
        
        # 编码器选择（统一策略）
        encoder = None
        recommended_codec = None
        codec_reason = None
        reasoning = ""
        
        if expected_codec:
            # 用户指定编码器（同图像/视频逻辑）
            encoder = expected_codec
            reasoning = f"User specified: {expected_codec}"
            log_info('AudioAI', f"User specified codec: {expected_codec}")
        elif not enable_format_recommendation:
            # 禁用推荐 → 源格式重编码（统一逻辑）
            encoder = source_codec
            reasoning = f"Re-encode with source codec: {source_codec}"
            log_info('AudioAI', f"Source format re-encoding: {source_codec}")
        else:
            # AI推荐：智能编码器升级
            if aggressive_mode:
                log_info('AudioAI', "Aggressive mode: prioritizing Opus")
            
            # 根据源编码器智能升级
            if source_codec in ['mp3', 'mp2']:
                # MP3 → AAC升级（保守策略，质量优先）
                if aggressive_mode:
                    encoder = 'opus'
                    recommended_codec = 'opus'
                    codec_reason = "MP3→Opus aggressive upgrade"
                    reasoning = f"Source: MP3 → Upgrade to Opus (30% better than MP3)"
                    log_info('AudioAI', "Aggressive upgrade: MP3 → Opus")
                else:
                    encoder = 'aac'
                    recommended_codec = 'aac'
                    codec_reason = "MP3→AAC safe upgrade"
                    reasoning = f"Source: MP3 → Upgrade to AAC (better quality)"
                    log_info('AudioAI', "Smart upgrade: MP3 → AAC")
            
            elif source_codec in ['aac', 'm4a']:
                # AAC → Opus（激进模式）或保持
                if aggressive_mode:
                    encoder = 'opus'
                    recommended_codec = 'opus'
                    codec_reason = "AAC→Opus aggressive upgrade"
                    reasoning = f"Source: AAC → Upgrade to Opus (cutting-edge)"
                    log_info('AudioAI', "Aggressive upgrade: AAC → Opus")
                else:
                    encoder = 'aac'
                    recommended_codec = 'aac'
                    codec_reason = "AAC already modern"
                    reasoning = f"Source: AAC → Keep AAC (already modern)"
            
            elif source_codec in ['opus']:
                # Opus已是最新
                encoder = 'opus'
                recommended_codec = 'opus'
                codec_reason = "Opus already cutting-edge"
                reasoning = f"Source: Opus → Keep Opus (already cutting-edge)"
            
            elif source_codec in ['flac', 'wav', 'pcm'] or source_codec.startswith('pcm_'):
                # 无损 → 根据模式选择（包括所有PCM变种：pcm_mulaw, pcm_s16le等）
                if optimize_mode == 'quality':
                    encoder = 'flac'
                    recommended_codec = 'flac'
                    codec_reason = "Lossless source, keep FLAC"
                    reasoning = f"Source: {source_codec.upper()} → FLAC (lossless preservation)"
                else:  # balanced - 默认AAC平衡策略
                    encoder = 'aac'
                    recommended_codec = 'aac'
                    codec_reason = "Lossless→AAC balanced"
                    reasoning = f"Source: {source_codec.upper()} → AAC (balanced quality-size)"
            
            elif source_codec in ['vorbis', 'ogg']:
                # Vorbis → Opus升级
                encoder = 'opus'
                recommended_codec = 'opus'
                codec_reason = "Vorbis→Opus upgrade"
                reasoning = f"Source: Vorbis → Upgrade to Opus (next-gen)"
                log_info('AudioAI', "Smart upgrade: Vorbis → Opus")
            
            else:
                # 未知编码器 → 默认策略
                if optimize_mode == 'quality':
                    encoder = 'flac'
                    reasoning = "Quality mode: FLAC for best quality"
                else:  # balanced - 默认策略
                    encoder = 'aac'
                    reasoning = "Balanced mode: AAC compatibility"
        
        # 比特率推荐（基于编码器和模式 + 源比特率）
        # 🎯 核心原则：质量不变前提下，必然减小大小
        if encoder == 'opus':
            # Opus比MP3效率高约30%，比AAC效率高约15%
            if source_codec in ['mp3']:
                # MP3 → Opus: 可以降低30%比特率保持相同质量
                target_bitrate = max(64, int(bitrate * 0.7))
                bitrate_reason = f"Opus upgrade from MP3 (30% more efficient)"
            elif source_codec in ['aac']:
                # AAC → Opus: 可以降低15%比特率
                target_bitrate = max(64, int(bitrate * 0.85))
                bitrate_reason = f"Opus upgrade from AAC (15% more efficient)"
            elif source_codec in ['vorbis', 'ogg']:
                # Vorbis → Opus: Opus比Vorbis效率高约20-25%
                # 🎯 核心原则：质量不变前提下，必然减小大小
                target_bitrate = max(64, int(bitrate * 0.75))
                bitrate_reason = f"Opus upgrade from Vorbis (25% more efficient, {bitrate}→{target_bitrate}kbps)"
            else:
                # 默认Opus策略
                if optimize_mode == 'quality':
                    target_bitrate = min(160, max(96, bitrate))
                    bitrate_reason = "Opus high quality"
                else:  # balanced
                    target_bitrate = 112
                    bitrate_reason = "Opus balanced"
        
        elif encoder == 'aac':
            # AAC比MP3效率高约15%
            if source_codec in ['mp3']:
                # MP3 → AAC: 可以降低15%比特率保持相同质量
                # 🎯 核心原则：质量不变前提下，不增加比特率
                calculated_bitrate = int(bitrate * 0.85)
                # 对于极低码率源文件，保持原比特率或略微降低，绝不增加
                if bitrate < 80:
                    target_bitrate = min(bitrate, max(calculated_bitrate, 64))  # 保持≤源比特率，最低64kbps
                    bitrate_reason = f"AAC upgrade from low-bitrate MP3 (preserve bitrate, {bitrate}→{target_bitrate}kbps)"
                else:
                    target_bitrate = max(80, calculated_bitrate)  # 正常范围最低80kbps
                    bitrate_reason = f"AAC upgrade from MP3 (15% more efficient, {bitrate}→{target_bitrate}kbps)"
            elif source_codec in ['flac', 'wav', 'pcm'] or source_codec.startswith('pcm_'):
                # 无损 → AAC: 基于源比特率智能选择（包括所有PCM变种）
                # 🎯 核心原则：维持质量前提下，必然减小空间占用
                if bitrate > 500:  # 高比特率无损源（如CD音质WAV 1411kbps）
                    target_bitrate = 192  # 高质量AAC足以保持透明度
                    bitrate_reason = f"AAC from high-bitrate lossless ({bitrate}→{target_bitrate}kbps, 透明压缩)"
                elif bitrate > 0:  # 有效比特率
                    # 🎯 绝对不超过源比特率（否则违反原则）
                    target_bitrate = min(bitrate, max(64, int(bitrate * 0.85)))
                    bitrate_reason = f"AAC from lossless (≤源比特率, {bitrate}→{target_bitrate}kbps)"
                else:  # 无效比特率（元数据缺失），保守默认
                    target_bitrate = 160
                    bitrate_reason = "AAC default (no source bitrate info)"
            else:
                # 其他格式默认AAC策略
                if optimize_mode == 'quality':
                    target_bitrate = min(224, max(128, bitrate)) if bitrate > 0 else 192
                    bitrate_reason = "AAC high quality"
                else:  # balanced
                    target_bitrate = max(128, min(160, bitrate)) if bitrate > 0 else 160
                    bitrate_reason = "AAC balanced"
        
        elif encoder == 'flac':
            # FLAC: 无损，比特率取决于源
            target_bitrate = max(bitrate, 800)  # 至少800kbps
            bitrate_reason = "FLAC lossless"
        
        elif encoder == 'mp3':
            # MP3: 质量256-320, 平衡192-256
            if optimize_mode == 'quality':
                target_bitrate = 320
                bitrate_reason = "MP3 maximum quality"
            else:  # balanced
                target_bitrate = 224
                bitrate_reason = "MP3 balanced"
        
        else:
            # 默认策略
            target_bitrate = 160
            bitrate_reason = "Default bitrate"
        
        # 预处理建议（移除所有降质预处理，保持质量优先）
        preprocessing_steps = []
        # 不再进行降低采样率或降低声道数的操作，维持质量
        
        # 置信度估算
        confidence = 0.85 if expected_codec else 0.75
        
        full_reasoning = f"{reasoning} | Bitrate={target_bitrate}kbps ({bitrate_reason})"
        
        log_info('AudioAI', f"Recommendation: {encoder} {target_bitrate}kbps, confidence={confidence:.1%}")
        if recommended_codec:
            log_info('AudioAI', f"AI recommended codec: {recommended_codec} ({codec_reason})")
        if preprocessing_steps:
            log_info('AudioAI', f"Preprocessing: {len(preprocessing_steps)} step(s)")
        
        # 统一返回结构（与图像/视频一致）
        return {
            'encoder': encoder,
            'bitrate': target_bitrate,
            'sample_rate': sample_rate,
            'channels': channels,
            'confidence': confidence,
            'reasoning': full_reasoning,
            'source_codec': source_codec,
            'recommended_codec': recommended_codec,
            'codec_reason': codec_reason,
            'preprocessing_steps': preprocessing_steps,
            'magika_detection': magika_result,  # 🆕 AI检测结果
            'metadata': {
                'duration': duration,
                'bitrate': bitrate,
                'sample_rate': sample_rate,
                'channels': channels,
                'codec': source_codec
            }
        }
