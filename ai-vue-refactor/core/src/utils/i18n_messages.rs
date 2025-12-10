//! 🌐 国际errormessage System
//! 
//! provide多语言error and warningmessagesupport

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// support语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
 /// 英语
    English,
 /// 简体文
    SimplifiedChinese,
 /// 繁体文
    TraditionalChinese,
 /// 日语
    Japanese,
}

impl Language {
 /// from语言代码create
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "en" | "en-us" | "en-gb" => Some(Self::English),
            "zh" | "zh-cn" | "zh-hans" => Some(Self::SimplifiedChinese),
            "zh-tw" | "zh-hant" => Some(Self::TraditionalChinese),
            "ja" | "ja-jp" => Some(Self::Japanese),
            _ => None,
        }
    }
    
 /// get语言代码
    pub fn code(&self) -> &'static str {
        match self {
            Self::English => "en",
            Self::SimplifiedChinese => "zh-CN",
            Self::TraditionalChinese => "zh-TW",
            Self::Japanese => "ja",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

/// errormessage键
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKey {
    // filevalidationerror
    FileNotExist,
    FileSizeZero,
    FileExtensionMissing,
    FilePathInvalid,
    FileMetadataError,
    FileTooLarge,
    FilenameIllegalChars,
    
    // formatvalidationerror  
    FormatInvalid,
    FormatNotDetected,
    
    // parametervalidationerror
    QualityInvalid,
    SpeedInvalid,
    
 // formatcompatibility性warning
    HeicNoTransparency,
    HeicNoAnimation,
    AvifAnimationLimited,
    JpegNoLossless,
    JxlQualityLow,
    
 // 模式validation
    ManualModeNoFormat,
    ManualModeNoQuality,
    SmartModeAiPredict,
    
    // outputvalidation
    OutputNotGenerated,
    OutputSizeZero,
    OutputSizeAbnormalSmall,
    OutputSizeAbnormalLarge,
    
 // 通用message
    NoFilesSelected,
    ValidationLevelPassed,
    ValidationLevelFailed,
    AllValidationPassed,
}

/// 国际message Manager
pub struct I18nMessages {
    messages: HashMap<(Language, MessageKey), String>,
    current_language: Language,
}

impl I18nMessages {
 /// create新message Manager
    pub fn new() -> Self {
        let mut messages = HashMap::new();
        
 // initialization所 has 语言message
        Self::init_english(&mut messages);
        Self::init_simplified_chinese(&mut messages);
        Self::init_traditional_chinese(&mut messages);
        Self::init_japanese(&mut messages);
        
        Self {
            messages,
            current_language: Language::default(),
        }
    }
    
 /// settingcurrent语言
    pub fn set_language(&mut self, lang: Language) {
        self.current_language = lang;
    }
    
 /// 获cancel息
    pub fn get(&self, key: MessageKey) -> String {
        self.messages
            .get(&(self.current_language, key))
            .cloned()
            .unwrap_or_else(|| {
 // 回退to英语
                self.messages
                    .get(&(Language::English, key))
                    .cloned()
                    .unwrap_or_else(|| format!("Message not found: {:?}", key))
            })
    }
    
 /// formatmessage（supportparameter）
    pub fn format(&self, key: MessageKey, args: &[(&str, &str)]) -> String {
        let mut msg = self.get(key);
        
        for (placeholder, value) in args {
            msg = msg.replace(&format!("{{{}}}", placeholder), value);
        }
        
        msg
    }
    
    // ========================================
 // 英语message
    // ========================================
    fn init_english(msgs: &mut HashMap<(Language, MessageKey), String>) {
        let lang = Language::English;
        
        // filevalidation
        msgs.insert((lang, MessageKey::FileNotExist), "❌ File #{num} ({name}): File does not exist".to_string());
        msgs.insert((lang, MessageKey::FileSizeZero), "⚠️ File #{num} ({name}): File size is 0".to_string());
        msgs.insert((lang, MessageKey::FileExtensionMissing), "❌ File #{num} ({name}): Missing file extension".to_string());
        msgs.insert((lang, MessageKey::FilePathInvalid), "❌ File #{num}: Invalid file path".to_string());
        msgs.insert((lang, MessageKey::FileMetadataError), "❌ File #{num} ({name}): Cannot read file metadata: {error}".to_string());
        msgs.insert((lang, MessageKey::FileTooLarge), "⚠️ File #{num} ({name}): File too large ({size} GB)".to_string());
        msgs.insert((lang, MessageKey::FilenameIllegalChars), "⚠️ File #{num} ({name}): Filename contains illegal characters".to_string());
        
        // formatvalidation
        msgs.insert((lang, MessageKey::FormatInvalid), "❌ Invalid target format: {format}".to_string());
        msgs.insert((lang, MessageKey::FormatNotDetected), "Unable to detect file format".to_string());
        
        // parametervalidation
        msgs.insert((lang, MessageKey::QualityInvalid), "❌ Invalid quality parameter: {quality} (should be 1-100)".to_string());
        msgs.insert((lang, MessageKey::SpeedInvalid), "❌ Invalid speed parameter: {speed} (should be 0-10)".to_string());
        
 // compatibility性warning
        msgs.insert((lang, MessageKey::HeicNoTransparency), "⚠️ HEIC does not support transparency, transparent backgrounds may change color".to_string());
        msgs.insert((lang, MessageKey::HeicNoAnimation), "⚠️ HEIC does not support animation, animation effects will be lost".to_string());
        msgs.insert((lang, MessageKey::AvifAnimationLimited), "⚠️ AVIF animation support is limited, may require special encoder".to_string());
        msgs.insert((lang, MessageKey::JpegNoLossless), "❌ JPEG format does not support lossless mode".to_string());
        msgs.insert((lang, MessageKey::JxlQualityLow), "⚠️ JXL quality parameter is low ({quality}), may affect visual quality".to_string());
        
 // 模式validation
        msgs.insert((lang, MessageKey::ManualModeNoFormat), "❌ Target format must be specified in manual mode".to_string());
        msgs.insert((lang, MessageKey::ManualModeNoQuality), "⚠️ Quality parameter not set in manual mode, will use default value".to_string());
        msgs.insert((lang, MessageKey::SmartModeAiPredict), "ℹ️ Smart mode will use AI predicted parameters".to_string());
        
        // outputvalidation
        msgs.insert((lang, MessageKey::OutputNotGenerated), "❌ Output file not generated: {path}".to_string());
        msgs.insert((lang, MessageKey::OutputSizeZero), "❌ Output file size is 0: {path}".to_string());
        msgs.insert((lang, MessageKey::OutputSizeAbnormalSmall), "⚠️ Output file abnormally small ({percent}% of original)".to_string());
        msgs.insert((lang, MessageKey::OutputSizeAbnormalLarge), "⚠️ Output file abnormally large ({ratio}x of original)".to_string());
        
 // 通用
        msgs.insert((lang, MessageKey::NoFilesSelected), "❌ No files selected".to_string());
        msgs.insert((lang, MessageKey::ValidationLevelPassed), "✅ Level {level} passed: {name}".to_string());
        msgs.insert((lang, MessageKey::ValidationLevelFailed), "❌ Level {level} failed: {name}".to_string());
        msgs.insert((lang, MessageKey::AllValidationPassed), "🎉 All validation levels passed".to_string());
    }
    
    // ========================================
 // 简体文message
    // ========================================
    fn init_simplified_chinese(msgs: &mut HashMap<(Language, MessageKey), String>) {
        let lang = Language::SimplifiedChinese;
        
        // filevalidation
        msgs.insert((lang, MessageKey::FileNotExist), "❌ 文件 #{num} ({name}): 文件不存在".to_string());
        msgs.insert((lang, MessageKey::FileSizeZero), "⚠️  文件 #{num} ({name}): 文件大小为0".to_string());
        msgs.insert((lang, MessageKey::FileExtensionMissing), "❌ 文件 #{num} ({name}): 缺少文件扩展名".to_string());
        msgs.insert((lang, MessageKey::FilePathInvalid), "❌ 文件 #{num}: 文件路径无效".to_string());
        msgs.insert((lang, MessageKey::FileMetadataError), "❌ 文件 #{num} ({name}): 无法读取文件元数据: {error}".to_string());
        msgs.insert((lang, MessageKey::FileTooLarge), "⚠️ 文件 #{num} ({name}): 文件过大 ({size} GB)".to_string());
        msgs.insert((lang, MessageKey::FilenameIllegalChars), "⚠️ 文件 #{num} ({name}): 文件名包含非法字符".to_string());
        
        // formatvalidation
        msgs.insert((lang, MessageKey::FormatInvalid), "❌ 无效的目标格式: {format}".to_string());
        msgs.insert((lang, MessageKey::FormatNotDetected), "无法检测文件格式".to_string());
        
        // parametervalidation
        msgs.insert((lang, MessageKey::QualityInvalid), "❌ 无效的质量参数: {quality} (应为1-100)".to_string());
        msgs.insert((lang, MessageKey::SpeedInvalid), "❌ 无效的速度参数: {speed} (应为0-10)".to_string());
        
 // compatibility性warning
        msgs.insert((lang, MessageKey::HeicNoTransparency), "⚠️ HEIC 不支持透明度，透明背景可能会改变颜色".to_string());
        msgs.insert((lang, MessageKey::HeicNoAnimation), "⚠️ HEIC 不支持动画，动画效果将丢失".to_string());
        msgs.insert((lang, MessageKey::AvifAnimationLimited), "⚠️ AVIF 动画支持有限，可能需要特殊编码器".to_string());
        msgs.insert((lang, MessageKey::JpegNoLossless), "❌ JPEG 格式不支持无损模式".to_string());
        msgs.insert((lang, MessageKey::JxlQualityLow), "⚠️ JXL 质量参数较低 ({quality})，可能影响视觉质量".to_string());
        
 // 模式validation
        msgs.insert((lang, MessageKey::ManualModeNoFormat), "❌ 手动模式必须指定目标格式".to_string());
        msgs.insert((lang, MessageKey::ManualModeNoQuality), "⚠️ 手动模式未设置质量参数，将使用默认值".to_string());
        msgs.insert((lang, MessageKey::SmartModeAiPredict), "ℹ️ 智能模式将使用AI预测参数".to_string());
        
        // outputvalidation
        msgs.insert((lang, MessageKey::OutputNotGenerated), "❌ 输出文件未生成: {path}".to_string());
        msgs.insert((lang, MessageKey::OutputSizeZero), "❌ 输出文件大小为0: {path}".to_string());
        msgs.insert((lang, MessageKey::OutputSizeAbnormalSmall), "⚠️ 输出文件异常小 (原始文件的{percent}%)".to_string());
        msgs.insert((lang, MessageKey::OutputSizeAbnormalLarge), "⚠️ 输出文件异常大 (原始文件的{ratio}倍)".to_string());
        
 // 通用
        msgs.insert((lang, MessageKey::NoFilesSelected), "❌ 未选择文件".to_string());
        msgs.insert((lang, MessageKey::ValidationLevelPassed), "✅ 级别 {level} 通过: {name}".to_string());
        msgs.insert((lang, MessageKey::ValidationLevelFailed), "❌ 级别 {level} 失败: {name}".to_string());
        msgs.insert((lang, MessageKey::AllValidationPassed), "🎉 所有验证级别通过".to_string());
    }
    
    // ========================================
 // 繁体文message
    // ========================================
    fn init_traditional_chinese(msgs: &mut HashMap<(Language, MessageKey), String>) {
        let lang = Language::TraditionalChinese;
        
        // filevalidation
        msgs.insert((lang, MessageKey::FileNotExist), "❌ 檔案 #{num} ({name}): 檔案不存在".to_string());
        msgs.insert((lang, MessageKey::FileSizeZero), "⚠️ 檔案 #{num} ({name}): 檔案大小為0".to_string());
        msgs.insert((lang, MessageKey::FileExtensionMissing), "❌ 檔案 #{num} ({name}): 缺少檔案副檔名".to_string());
        msgs.insert((lang, MessageKey::FilePathInvalid), "❌ 檔案 #{num}: 檔案路徑無效".to_string());
        msgs.insert((lang, MessageKey::FileMetadataError), "❌ 檔案 #{num} ({name}): 無法讀取檔案中繼資料: {error}".to_string());
        msgs.insert((lang, MessageKey::FileTooLarge), "⚠️ 檔案 #{num} ({name}): 檔案過大 ({size} GB)".to_string());
        msgs.insert((lang, MessageKey::FilenameIllegalChars), "⚠️ 檔案 #{num} ({name}): 檔案名稱包含非法字元".to_string());
        
        // formatvalidation
        msgs.insert((lang, MessageKey::FormatInvalid), "❌ 無效的目標格式: {format}".to_string());
        msgs.insert((lang, MessageKey::FormatNotDetected), "無法檢測檔案格式".to_string());
        
        // parametervalidation
        msgs.insert((lang, MessageKey::QualityInvalid), "❌ 無效的質量參數: {quality} (應為1-100)".to_string());
       msgs.insert((lang, MessageKey::SpeedInvalid), "❌ 無效的速度參數: {speed} (應為0-10)".to_string());
        
 // compatibility性warning
        msgs.insert((lang, MessageKey::HeicNoTransparency), "⚠️ HEIC 不支援透明度，透明背景可能會改變顏色".to_string());
        msgs.insert((lang, MessageKey::HeicNoAnimation), "⚠️ HEIC 不支援動畫，動畫效果將遺失".to_string());
        msgs.insert((lang, MessageKey::AvifAnimationLimited), "⚠️ AVIF 動畫支援有限，可能需要特殊編碼器".to_string());
        msgs.insert((lang, MessageKey::JpegNoLossless), "❌ JPEG 格式不支援無損模式".to_string());
        msgs.insert((lang, MessageKey::JxlQualityLow), "⚠️ JXL 質量參數較低 ({quality})，可能影響視覺質量".to_string());
        
 // 模式validation
        msgs.insert((lang, MessageKey::ManualModeNoFormat), "❌ 手動模式必須指定目標格式".to_string());
        msgs.insert((lang, MessageKey::ManualModeNoQuality), "⚠️ 手動模式未設定質量參數，將使用預設值".to_string());
        msgs.insert((lang, MessageKey::SmartModeAiPredict), "ℹ️ 智能模式將使用AI預測參數".to_string());
        
        // outputvalidation
        msgs.insert((lang, MessageKey::OutputNotGenerated), "❌ 輸出檔案未產生: {path}".to_string());
        msgs.insert((lang, MessageKey::OutputSizeZero), "❌ 輸出檔案大小為0: {path}".to_string());
        msgs.insert((lang, MessageKey::OutputSizeAbnormalSmall), "⚠️ 輸出檔案異常小 (原始檔案的{percent}%)".to_string());
        msgs.insert((lang, MessageKey::OutputSizeAbnormalLarge), "⚠️ 輸出檔案異常大 (原始檔案的{ratio}倍)".to_string());
        
 // 通用
        msgs.insert((lang, MessageKey::NoFilesSelected), "❌ 未選擇檔案".to_string());
        msgs.insert((lang, MessageKey::ValidationLevelPassed), "✅ 級別 {level} 通過: {name}".to_string());
        msgs.insert((lang, MessageKey::ValidationLevelFailed), "❌ 級別 {level} 失敗: {name}".to_string());
        msgs.insert((lang, MessageKey::AllValidationPassed), "🎉 所有驗證級別通過".to_string());
    }
    
    // ========================================
 // 日语message
    // ========================================
    fn init_japanese(msgs: &mut HashMap<(Language, MessageKey), String>) {
        let lang = Language::Japanese;
        
        // filevalidation
        msgs.insert((lang, MessageKey::FileNotExist), "❌ ファイル #{num} ({name}): ファイルが存在しません".to_string());
        msgs.insert((lang, MessageKey::FileSizeZero), "⚠️ ファイル #{num} ({name}): ファイルサイズが0です".to_string());
        msgs.insert((lang, MessageKey::FileExtensionMissing), "❌ ファイル #{num} ({name}): ファイル拡張子がありません".to_string());
        msgs.insert((lang, MessageKey::FilePathInvalid), "❌ ファイル #{num}: 無効なファイルパス".to_string());
        msgs.insert((lang, MessageKey::FileMetadataError), "❌ ファイル #{num} ({name}): ファイルのメタデータを読み取れません: {error}".to_string());
        msgs.insert((lang, MessageKey::FileTooLarge), "⚠️ ファイル #{num} ({name}): ファイルサイズが大きすぎます ({size} GB)".to_string());
        msgs.insert((lang, MessageKey::FilenameIllegalChars), "⚠️ ファイル #{num} ({name}): ファイル名に不正な文字が含まれています".to_string());
        
        // formatvalidation
        msgs.insert((lang, MessageKey::FormatInvalid), "❌ 無効なターゲットフォーマット: {format}".to_string());
        msgs.insert((lang, MessageKey::FormatNotDetected), "ファイルフォーマットを検出できません".to_string());
        
        // parametervalidation
        msgs.insert((lang, MessageKey::QualityInvalid), "❌ 無効な品質パラメータ: {quality} (1-100であるべき)".to_string());
        msgs.insert((lang, MessageKey::SpeedInvalid), "❌ 無効な速度パラメータ: {speed} (0-10であるべき)".to_string());
        
 // compatibility性warning
        msgs.insert((lang, MessageKey::HeicNoTransparency), "⚠️ HEICは透明度をサポートしていません。透明な背景は色が変わる可能性があります".to_string());
        msgs.insert((lang, MessageKey::HeicNoAnimation), "⚠️ HEICはアニメーションをサポートしていません。アニメーション効果は失われます".to_string());
        msgs.insert((lang, MessageKey::AvifAnimationLimited), "⚠️ AVIFのアニメーションサポートは限定的で、特別なエンコーダーが必要な場合があります".to_string());
        msgs.insert((lang, MessageKey::JpegNoLossless), "❌ JPEG形式はロスレスモードをサポートしていません".to_string());
        msgs.insert((lang, MessageKey::JxlQualityLow), "⚠️ JXL品質パラメータが低い ({quality})、視覚品質に影響する可能性があります".to_string());
        
 // 模式validation
        msgs.insert((lang, MessageKey::ManualModeNoFormat), "❌ 手動モードではターゲットフォーマットを指定する必要があります".to_string());
        msgs.insert((lang, MessageKey::ManualModeNoQuality), "⚠️ 手動モードで品質パラメータが設定されていません。デフォルト値を使用します".to_string());
        msgs.insert((lang, MessageKey::SmartModeAiPredict), "ℹ️ スマートモードはAI予測パラメータを使用します".to_string());
        
        // outputvalidation
        msgs.insert((lang, MessageKey::OutputNotGenerated), "❌ 出力ファイルが生成されていません: {path}".to_string());
        msgs.insert((lang, MessageKey::OutputSizeZero), "❌ 出力ファイルのサイズが0です: {path}".to_string());
        msgs.insert((lang, MessageKey::OutputSizeAbnormalSmall), "⚠️ 出力ファイルが異常に小さい (元のファイルの{percent}%)".to_string());
        msgs.insert((lang, MessageKey::OutputSizeAbnormalLarge), "⚠️ 出力ファイルが異常に大きい (元のファイルの{ratio}倍)".to_string());
        
 // 通用
        msgs.insert((lang, MessageKey::NoFilesSelected), "❌ ファイルが選択されていません".to_string());
        msgs.insert((lang, MessageKey::ValidationLevelPassed), "✅ レベル {level} 合格: {name}".to_string());
        msgs.insert((lang, MessageKey::ValidationLevelFailed), "❌ レベル {level} 失敗: {name}".to_string());
        msgs.insert((lang, MessageKey::AllValidationPassed), "🎉 すべての検証レベルに合格しました".to_string());
    }
}

impl Default for I18nMessages {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("zh-CN"), Some(Language::SimplifiedChinese));
        assert_eq!(Language::from_code("zh-TW"), Some(Language::TraditionalChinese));
        assert_eq!(Language::from_code("ja"), Some(Language::Japanese));
        assert_eq!(Language::from_code("unknown"), None);
    }
    
    #[test]
    fn test_get_message_english() {
        let msgs = I18nMessages::new();
        let msg = msgs.get(MessageKey::NoFilesSelected);
        assert_eq!(msg, "❌ No files selected");
    }
    
    #[test]
    fn test_get_message_chinese() {
        let mut msgs = I18nMessages::new();
        msgs.set_language(Language::SimplifiedChinese);
        let msg = msgs.get(MessageKey::NoFilesSelected);
        assert_eq!(msg, "❌ 未选择文件");
    }
    
    #[test]
    fn test_format_message() {
        let msgs = I18nMessages::new();
        let msg = msgs.format(
            MessageKey::QualityInvalid,
            &[("quality", "150")]
        );
        assert!(msg.contains("150"));
        assert!(msg.contains("1-100"));
    }
    
    #[test]
    fn test_all_languages_have_messages() {
        let msgs = I18nMessages::new();
        let keys = [
            MessageKey::FileNotExist,
            MessageKey::FormatInvalid,
            MessageKey::NoFilesSelected,
        ];
        
        for lang in [Language::English, Language::SimplifiedChinese, 
                     Language::TraditionalChinese, Language::Japanese] {
            for key in keys {
                assert!(msgs.messages.contains_key(&(lang, key)));
            }
        }
    }
}
