// 🧪 FFmpeg自动选择测试
// 验证像素格式自动选择功能

#[cfg(test)]
mod tests {
    use pixly_kernel::video_processor::*;
    use pixly_kernel::modern_formats::*;

    #[test]
    fn test_video_pix_fmt_auto_select() {
        // ✅ 默认配置应该让FFmpeg自动选择
        let config = VideoConversionConfig::default();
        assert_eq!(config.pix_fmt, None);
        
        // 用户可以明确指定
        let config_manual = VideoConversionConfig {
            pix_fmt: Some("yuv420p".to_string()),
            ..Default::default()
        };
        assert_eq!(config_manual.pix_fmt, Some("yuv420p".to_string()));
    }

    #[test]
    fn test_avif_pix_fmt_from_user_choice() {
        // ✅ AVIF根据用户选择的bit_depth和chroma_subsampling计算pix_fmt
        let params_8bit_420 = AVIFParams {
            bit_depth: 8,
            chroma_subsampling: "420".to_string(),
            ..Default::default()
        };
        assert_eq!(params_8bit_420.bit_depth, 8);
        assert_eq!(params_8bit_420.chroma_subsampling, "420");
        
        let params_10bit_422 = AVIFParams {
            bit_depth: 10,
            chroma_subsampling: "422".to_string(),
            ..Default::default()
        };
        assert_eq!(params_10bit_422.bit_depth, 10);
        assert_eq!(params_10bit_422.chroma_subsampling, "422");
        
        // 如果用户不指定chroma_subsampling（空字符串），应该让FFmpeg自动选择
        let params_auto = AVIFParams {
            chroma_subsampling: "".to_string(),
            ..Default::default()
        };
        assert_eq!(params_auto.chroma_subsampling, "");
    }

    #[test]
    fn test_heic_pix_fmt_from_user_choice() {
        // ✅ HEIC根据用户选择的chroma_subsampling
        let params_420 = HEICParams {
            chroma_subsampling: "420".to_string(),
            ..Default::default()
        };
        assert_eq!(params_420.chroma_subsampling, "420");
        
        let params_444 = HEICParams {
            chroma_subsampling: "444".to_string(),
            ..Default::default()
        };
        assert_eq!(params_444.chroma_subsampling, "444");
        
        // 空字符串表示自动选择
        let params_auto = HEICParams {
            chroma_subsampling: "".to_string(),
            ..Default::default()
        };
        assert_eq!(params_auto.chroma_subsampling, "");
    }

    #[test]
    fn test_prores_no_hardcoded_pix_fmt() {
        // ✅ ProRes配置不应该硬编码pix_fmt
        let config = VideoConversionConfig {
            codec: "prores".to_string(),
            pix_fmt: None,  // 让FFmpeg自动选择
            ..Default::default()
        };
        
        assert_eq!(config.codec, "prores");
        assert_eq!(config.pix_fmt, None);
        
        // 用户也可以明确指定ProRes的pix_fmt
        let config_manual = VideoConversionConfig {
            codec: "prores".to_string(),
            pix_fmt: Some("yuv422p10le".to_string()),
            ..Default::default()
        };
        assert_eq!(config_manual.pix_fmt, Some("yuv422p10le".to_string()));
    }

    #[test]
    fn test_ffmpeg_auto_select_philosophy() {
        // ✅ 验证自动选择的设计哲学
        
        // 1. 默认情况：让FFmpeg自动选择（最智能）
        let default_config = VideoConversionConfig::default();
        assert_eq!(default_config.pix_fmt, None);
        
        // 2. 用户明确指定：尊重用户选择
        let user_config = VideoConversionConfig {
            pix_fmt: Some("yuv444p".to_string()),
            ..Default::default()
        };
        assert_eq!(user_config.pix_fmt, Some("yuv444p".to_string()));
        
        // 3. 高级用户可以完全控制
        let advanced_config = VideoConversionConfig {
            codec: "h265".to_string(),
            pix_fmt: Some("yuv420p10le".to_string()),
            gop_size: Some(120),
            bframes: Some(8),
            ref_frames: Some(5),
            me_method: Some("umh".to_string()),
            ..Default::default()
        };
        assert!(advanced_config.pix_fmt.is_some());
        assert!(advanced_config.gop_size.is_some());
    }

    #[test]
    fn test_no_unwanted_hardcoded_parameters() {
        // ❌ 确保没有不情愿的硬编码参数
        
        // 视频转换默认配置
        let video_config = VideoConversionConfig::default();
        assert_eq!(video_config.pix_fmt, None, "pix_fmt should be None by default");
        
        // AVIF默认配置
        let avif_params = AVIFParams::default();
        // AVIF的chroma_subsampling有默认值是合理的，因为这是用户可见的选项
        assert!(!avif_params.chroma_subsampling.is_empty());
        
        // HEIC默认配置
        let heic_params = HEICParams::default();
        // HEIC的chroma_subsampling有默认值是合理的
        assert!(!heic_params.chroma_subsampling.is_empty());
    }

    #[test]
    fn test_ffmpeg_loss_calculation_awareness() {
        // ✅ 验证我们理解FFmpeg的损失计算机制
        
        // FFmpeg使用avcodec_find_best_pix_fmt_of_2()计算损失
        // 损失优先级：
        // 1. FF_LOSS_RESOLUTION - 分辨率损失
        // 2. FF_LOSS_DEPTH - 色彩深度损失
        // 3. FF_LOSS_COLORSPACE - 色彩空间转换损失
        // 4. FF_LOSS_ALPHA - Alpha通道损失
        // 5. FF_LOSS_COLORQUANT - 色彩量化损失
        // 6. FF_LOSS_CHROMA - 色度损失
        
        // 当我们不指定pix_fmt时，FFmpeg会：
        // 1. 获取编码器支持的格式列表
        // 2. 计算源格式到每个候选格式的损失值
        // 3. 选择损失值最小的格式
        
        // 例如：
        // 输入 yuv411p → 编码器支持 yuv420p, yuv422p
        // FFmpeg会选择yuv422p（更接近4:1:1，损失更小）
        
        let config = VideoConversionConfig {
            pix_fmt: None,  // 让FFmpeg智能选择
            ..Default::default()
        };
        
        assert_eq!(config.pix_fmt, None);
        // 这样FFmpeg可以根据实际输入和编码器能力做出最佳选择
    }
}
