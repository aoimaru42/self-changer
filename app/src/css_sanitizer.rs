/// シンプルなCSSサニタイザー
/// 最低限のセキュリティ対策のみを行う
pub struct CssSanitizer;

impl CssSanitizer {
    /// 新しいCssSanitizerインスタンスを作成
    pub fn new() -> Self {
        Self
    }

    /// CSS文字列をサニタイズ（シンプル版）
    pub fn sanitize_css_string(&self, css_string: &str) -> String {
        // 危険な文字列をチェックして除去
        let dangerous_patterns = vec![
            "javascript:",
            "vbscript:",
            "expression(",
            "eval(",
            "alert(",
            "document.",
            "window.",
            "location.",
            "history.",
            "navigator.",
            "<script",
            "</script",
            "onload=",
            "onerror=",
            "onclick=",
            "onmouseover=",
            "url(javascript:",
            "url(vbscript:",
            "url(data:text/html",
            "behavior:",
            "binding:",
            "-moz-binding:",
            "-webkit-binding:",
        ];

        let mut sanitized = css_string.to_string();

        // 危険なパターンを除去
        for pattern in dangerous_patterns {
            sanitized = sanitized.replace(pattern, "");
        }

        // 基本的なクリーンアップ
        sanitized = sanitized.trim().to_string();

        // 空の場合は空文字列を返す
        if sanitized.is_empty() {
            return String::new();
        }

        // 最後のセミコロンを除去（統一性のため）
        if sanitized.ends_with(';') {
            sanitized.pop();
        }

        sanitized
    }
}

impl Default for CssSanitizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_safe_properties() {
        let sanitizer = CssSanitizer::new();

        assert_eq!(
            sanitizer.sanitize_css_string("background-color: #ff0000; color: white;"),
            "background-color: #ff0000; color: white"
        );

        assert_eq!(
            sanitizer.sanitize_css_string("width: 100px; height: 200px; margin: 10px;"),
            "width: 100px; height: 200px; margin: 10px"
        );
    }

    #[test]
    fn test_block_dangerous_properties() {
        let sanitizer = CssSanitizer::new();

        // 危険なパターンが除去されることを確認
        let result1 =
            sanitizer.sanitize_css_string("background-color: red; javascript: alert('xss');");
        assert!(result1.contains("background-color: red"));
        assert!(!result1.contains("javascript:"));

        let result2 = sanitizer.sanitize_css_string("expression(alert('xss')); color: blue;");
        assert!(result2.contains("color: blue"));
        assert!(!result2.contains("expression("));
    }

    #[test]
    fn test_sanitize_values() {
        let sanitizer = CssSanitizer::new();

        assert_eq!(
            sanitizer.sanitize_css_string("color: #ffffff; background-color: rgb(255, 0, 0);"),
            "color: #ffffff; background-color: rgb(255, 0, 0)"
        );

        assert_eq!(
            sanitizer.sanitize_css_string("font-family: Arial, sans-serif; font-size: 16px;"),
            "font-family: Arial, sans-serif; font-size: 16px"
        );
    }
}
