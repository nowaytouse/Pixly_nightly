

import subprocess
import json
from pathlib import Path

try:
    from pixly_logging import log_info, log_warn, log_debug
except ImportError:
    def log_info(comp, msg, **ctx): print(f"✅ [{comp}] {msg}")
    def log_warn(comp, msg, **ctx): print(f"⚠️  [{comp}] {msg}")

try:
    from magika import Magika
    MAGIKA_AVAILABLE = True
except ImportError:
    MAGIKA_AVAILABLE = False

class AudioPredictor:

    def __init__(self):
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
        
        options = options or {}
        expected_codec = options.get('expected_codec', None)
        enable_format_recommendation = options.get('enable_format_recommendation', True)
        enable_preprocessing = options.get('enable_preprocessing', True)
        aggressive_mode = options.get('aggressive_mode', False)
        enable_magika = options.get('enable_magika', True)

        log_info('AudioAI', f"Analyzing audio: {audio_path}")

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

                if not magika_result['is_audio']:
                    log_warn('AudioAI', f"File is not audio: {magika_result['detected_type']}")
            except Exception as e:
                log_warn('AudioAI', f"Magika detection failed: {e}")

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
            bitrate = int(format_info.get('bit_rate', 0)) // 1000

            log_info('AudioAI', f"Metadata: {source_codec}, {sample_rate}Hz, {channels}ch, {bitrate}kbps, {duration:.2f}s, {file_size}bytes")

            if file_size > 0 and file_size < 10240:
                log_warn('AudioAI', f"Info")
                return {
                    'success': False,
                    'skip_reason': 'tiny_file',
                    'message': f'File too small ({file_size}bytes), conversion overhead may exceed compression benefit',
                    'recommended_action': 'keep_original'
                }
            if duration > 0 and duration < 1.0:
                log_warn('AudioAI', f"Info")
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

        encoder = None
        recommended_codec = None
        codec_reason = None
        reasoning = ""

        if expected_codec:
            encoder = expected_codec
            reasoning = f"User specified: {expected_codec}"
            log_info('AudioAI', f"User specified codec: {expected_codec}")
        elif not enable_format_recommendation:
            encoder = source_codec
            reasoning = f"Re-encode with source codec: {source_codec}"
            log_info('AudioAI', f"Source format re-encoding: {source_codec}")
        else:
            if aggressive_mode:
                log_info('AudioAI', "Aggressive mode: prioritizing Opus")

            if source_codec in ['mp3', 'mp2']:
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
                encoder = 'opus'
                recommended_codec = 'opus'
                codec_reason = "Opus already cutting-edge"
                reasoning = f"Source: Opus → Keep Opus (already cutting-edge)"

            elif source_codec in ['flac', 'wav', 'pcm'] or source_codec.startswith('pcm_'):
                if optimize_mode == 'quality':
                    encoder = 'flac'
                    recommended_codec = 'flac'
                    codec_reason = "Lossless source, keep FLAC"
                    reasoning = f"Source: {source_codec.upper()} → FLAC (lossless preservation)"
                else:
                    encoder = 'aac'
                    recommended_codec = 'aac'
                    codec_reason = "Lossless→AAC balanced"
                    reasoning = f"Source: {source_codec.upper()} → AAC (balanced quality-size)"

            elif source_codec in ['vorbis', 'ogg']:
                encoder = 'opus'
                recommended_codec = 'opus'
                codec_reason = "Vorbis→Opus upgrade"
                reasoning = f"Source: Vorbis → Upgrade to Opus (next-gen)"
                log_info('AudioAI', "Smart upgrade: Vorbis → Opus")

            else:
                if optimize_mode == 'quality':
                    encoder = 'flac'
                    reasoning = "Quality mode: FLAC for best quality"
                else:
                    encoder = 'aac'
                    reasoning = "Balanced mode: AAC compatibility"

        if encoder == 'opus':
            if source_codec in ['mp3']:
                target_bitrate = max(64, int(bitrate * 0.7))
                bitrate_reason = f"Opus upgrade from MP3 (30% more efficient)"
            elif source_codec in ['aac']:
                target_bitrate = max(64, int(bitrate * 0.85))
                bitrate_reason = f"Opus upgrade from AAC (15% more efficient)"
            elif source_codec in ['vorbis', 'ogg']:
                target_bitrate = max(64, int(bitrate * 0.75))
                bitrate_reason = f"Opus upgrade from Vorbis (25% more efficient, {bitrate}→{target_bitrate}kbps)"
            else:
                if optimize_mode == 'quality':
                    target_bitrate = min(160, max(96, bitrate))
                    bitrate_reason = "Opus high quality"
                else:
                    target_bitrate = 112
                    bitrate_reason = "Opus balanced"

        elif encoder == 'aac':
            if source_codec in ['mp3']:
                calculated_bitrate = int(bitrate * 0.85)
                if bitrate < 80:
                    target_bitrate = min(bitrate, max(calculated_bitrate, 64))
                    bitrate_reason = f"AAC upgrade from low-bitrate MP3 (preserve bitrate, {bitrate}→{target_bitrate}kbps)"
                else:
                    target_bitrate = max(80, calculated_bitrate)
                    bitrate_reason = f"AAC upgrade from MP3 (15% more efficient, {bitrate}→{target_bitrate}kbps)"
            elif source_codec in ['flac', 'wav', 'pcm'] or source_codec.startswith('pcm_'):
                if bitrate > 500:
                    target_bitrate = 192
                    bitrate_reason = f"Info"
                elif bitrate > 0:
                    target_bitrate = min(bitrate, max(64, int(bitrate * 0.85)))
                    bitrate_reason = f"Info"
                else:
                    target_bitrate = 160
                    bitrate_reason = "AAC default (no source bitrate info)"
            else:
                if optimize_mode == 'quality':
                    target_bitrate = min(224, max(128, bitrate)) if bitrate > 0 else 192
                    bitrate_reason = "AAC high quality"
                else:
                    target_bitrate = max(128, min(160, bitrate)) if bitrate > 0 else 160
                    bitrate_reason = "AAC balanced"

        elif encoder == 'flac':
            target_bitrate = max(bitrate, 800)
            bitrate_reason = "FLAC lossless"

        elif encoder == 'mp3':
            if optimize_mode == 'quality':
                target_bitrate = 320
                bitrate_reason = "MP3 maximum quality"
            else:
                target_bitrate = 224
                bitrate_reason = "MP3 balanced"

        else:
            target_bitrate = 160
            bitrate_reason = "Default bitrate"

        preprocessing_steps = []

        confidence = 0.85 if expected_codec else 0.75

        full_reasoning = f"{reasoning} | Bitrate={target_bitrate}kbps ({bitrate_reason})"

        log_info('AudioAI', f"Recommendation: {encoder} {target_bitrate}kbps, confidence={confidence:.1%}")
        if recommended_codec:
            log_info('AudioAI', f"AI recommended codec: {recommended_codec} ({codec_reason})")
        if preprocessing_steps:
            log_info('AudioAI', f"Preprocessing: {len(preprocessing_steps)} step(s)")

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
            'magika_detection': magika_result,
            'metadata': {
                'duration': duration,
                'bitrate': bitrate,
                'sample_rate': sample_rate,
                'channels': channels,
                'codec': source_codec
            }
        }
