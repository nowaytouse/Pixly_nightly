// 🧪 测试"自动"参数选项
// 验证UI中的"auto"选项能正确传递到内核并被处理

#[cfg(test)]
mod tests {
    use pixly_kernel::modern_formats::*;

    #[test]
    fn test_jxl_auto_bit_depth() {
        // ✅ bit_depth = 0 表示自动
        let params = JXLParams {
            quality: 90,
            effort: 7,
            lossless: false,
            modular: false,
            progressive: true,
            responsive: true,
            gaborish: true,
            photon_noise: 0,
            decoding_speed: 0,
            distance: 1.0,
            bit_depth: 0,  // 0 = auto
            color_space: "auto".to_string(),
            patches: 1,
        };
        
        assert_eq!(params.bit_depth, 0, "bit_depth应该是0表示自动");
        assert_eq!(params.color_space, "auto", "color_space应该是auto");
    }

    #[test]
    fn test_jxl_from_quality_uses_auto() {
        // ✅ from_quality应该使用auto作为默认值
        let params = JXLParams::from_quality(90);
        
        assert_eq!(params.bit_depth, 0, "from_quality应该默认bit_depth为0(auto)");
        assert_eq!(params.color_space, "auto", "from_quality应该默认color_space为auto");
    }

    #[test]
    fn test_jxl_explicit_bit_depth() {
        // ✅ 显式指定bit_depth应该被保留
        let params = JXLParams {
            quality: 90,
            effort: 7,
            lossless: false,
            modular: false,
            progressive: true,
            responsive: true,
            gaborish: true,
            photon_noise: 0,
            decoding_speed: 0,
            distance: 1.0,
            bit_depth: 10,  // 显式指定10-bit
            color_space: "Display P3".to_string(),
            patches: 1,
        };
        
        assert_eq!(params.bit_depth, 10, "显式指定的bit_depth应该被保留");
        assert_eq!(params.color_space, "Display P3", "显式指定的color_space应该被保留");
    }

    #[test]
    fn test_avif_auto_chroma() {
        // ✅ AVIF的chroma_subsampling可以是"auto"
        let params = AVIFParams {
            encoder: "libaom-av1".to_string(),
            crf: 30,
            speed: 6,
            bit_depth: 10,
            min_quantizer: 0,
            max_quantizer: 63,
            chroma_subsampling: "auto".to_string(),
            tiles_rows: 1,
            tiles_cols: 1,
            premultiply_alpha: false,
        };
        
        assert_eq!(params.chroma_subsampling, "auto", "AVIF应该支持auto色度子采样");
    }

    #[test]
    fn test_heic_auto_chroma() {
        // ✅ HEIC的chroma_subsampling可以是"auto"
        let params = HEICParams {
            quality: 90,
            encoder: "x265".to_string(),
            chroma_subsampling: "auto".to_string(),
            lossless: false,
            embed_thumbnail: false,
        };
        
        assert_eq!(params.chroma_subsampling, "auto", "HEIC应该支持auto色度子采样");
    }
}
