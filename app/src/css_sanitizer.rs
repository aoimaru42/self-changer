use std::collections::HashSet;

/// CSSプロパティのサニタイズとバリデーションを行う構造体
pub struct CssSanitizer {
    /// 許可されたCSSプロパティのセット
    allowed_properties: HashSet<String>,
    /// 危険なCSSプロパティのセット（禁止）
    forbidden_properties: HashSet<String>,
}

impl CssSanitizer {
    /// 新しいCssSanitizerインスタンスを作成
    pub fn new() -> Self {
        let mut allowed_properties = HashSet::new();
        
        // 許可されたCSSプロパティを追加
        let properties = vec![
            // レイアウト
            "display", "position", "top", "right", "bottom", "left", "z-index",
            "width", "height", "max-width", "max-height", "min-width", "min-height",
            "margin", "margin-top", "margin-right", "margin-bottom", "margin-left",
            "padding", "padding-top", "padding-right", "padding-bottom", "padding-left",
            "border", "border-width", "border-style", "border-color",
            "border-top", "border-right", "border-bottom", "border-left",
            "border-radius", "border-top-left-radius", "border-top-right-radius",
            "border-bottom-left-radius", "border-bottom-right-radius",
            
            // フレックスボックス
            "flex", "flex-direction", "flex-wrap", "flex-grow", "flex-shrink", "flex-basis",
            "justify-content", "align-items", "align-self", "align-content",
            "gap", "row-gap", "column-gap",
            
            // グリッド
            "grid", "grid-template-columns", "grid-template-rows", "grid-template-areas",
            "grid-column", "grid-row", "grid-area", "grid-gap", "grid-row-gap", "grid-column-gap",
            
            // 背景
            "background", "background-color", "background-image", "background-repeat",
            "background-position", "background-size", "background-attachment",
            
            // テキスト
            "color", "font-family", "font-size", "font-weight", "font-style", "font-variant",
            "line-height", "text-align", "text-decoration", "text-transform", "text-shadow",
            "letter-spacing", "word-spacing", "white-space", "word-wrap", "word-break",
            
            // 視覚効果
            "opacity", "visibility", "overflow", "overflow-x", "overflow-y",
            "box-shadow", "text-shadow", "filter", "backdrop-filter",
            "transform", "transform-origin", "perspective", "perspective-origin",
            
            // アニメーション
            "transition", "transition-property", "transition-duration", "transition-timing-function",
            "animation", "animation-name", "animation-duration", "animation-timing-function",
            "animation-delay", "animation-iteration-count", "animation-direction", "animation-fill-mode",
            
            // その他
            "cursor", "user-select", "pointer-events", "resize", "outline", "outline-offset",
        ];
        
        for prop in properties {
            allowed_properties.insert(prop.to_string());
        }
        
        let mut forbidden_properties = HashSet::new();
        let forbidden = vec![
            // 危険なプロパティ
            "expression", "javascript", "vbscript", "data", "url",
            "behavior", "binding", "-moz-binding", "-webkit-binding",
            "import", "charset", "namespace", "page", "font-face",
            // セキュリティリスクのあるプロパティ
            "content", "counter-reset", "counter-increment", "quotes",
            "clip-path", "mask", "mask-image", "mask-position", "mask-size",
            "clip", "clip-path", "shape-outside", "shape-margin",
        ];
        
        for prop in forbidden {
            forbidden_properties.insert(prop.to_string());
        }
        
        Self {
            allowed_properties,
            forbidden_properties,
        }
    }
    
    /// CSSプロパティと値をサニタイズ
    pub fn sanitize_css_property(&self, property: &str, value: &str) -> Option<String> {
        let property_lower = property.to_lowercase();
        
        // 禁止されたプロパティをチェック
        if self.forbidden_properties.contains(&property_lower) {
            return None;
        }
        
        // 許可されたプロパティをチェック
        if !self.allowed_properties.contains(&property_lower) {
            return None;
        }
        
        // 値のサニタイズ
        let sanitized_value = self.sanitize_value(value)?;
        
        Some(format!("{}: {}", property_lower, sanitized_value))
    }
    
    /// CSS値をサニタイズ
    fn sanitize_value(&self, value: &str) -> Option<String> {
        let value = value.trim();
        
        // 空の値は許可しない
        if value.is_empty() {
            return None;
        }
        
        // 危険な文字列をチェック
        if self.contains_dangerous_content(value) {
            return None;
        }
        
        // 基本的な値のバリデーション
        if self.is_valid_css_value(value) {
            Some(value.to_string())
        } else {
            None
        }
    }
    
    /// 危険なコンテンツが含まれているかチェック
    fn contains_dangerous_content(&self, value: &str) -> bool {
        let dangerous_patterns = vec![
            "javascript:", "vbscript:", "data:", "expression(", "eval(", "alert(",
            "document.", "window.", "location.", "history.", "navigator.",
            "<script", "</script", "onload=", "onerror=", "onclick=", "onmouseover=",
            "url(javascript:", "url(vbscript:", "url(data:text/html",
            "behavior:", "binding:", "-moz-binding:", "-webkit-binding:",
        ];
        
        let value_lower = value.to_lowercase();
        dangerous_patterns.iter().any(|pattern| value_lower.contains(pattern))
    }
    
    /// CSS値が有効かチェック（簡略化版）
    fn is_valid_css_value(&self, value: &str) -> bool {
        // 基本的な値のパターンをチェック
        if value.contains("javascript:") || value.contains("vbscript:") || value.contains("data:") {
            return false;
        }
        
        // 基本的なCSS値のパターン
        if value.contains("px") || value.contains("em") || value.contains("rem") || 
           value.contains("%") || value.contains("vh") || value.contains("vw") ||
           value.contains("auto") || value.contains("inherit") || value.contains("initial") ||
           value.contains("unset") || value.contains("none") || value.contains("transparent") ||
           value.contains("#") || value.contains("rgb") || value.contains("hsl") ||
           value.contains("bold") || value.contains("italic") || value.contains("normal") ||
           value.contains("left") || value.contains("right") || value.contains("center") ||
           value.contains("top") || value.contains("bottom") || value.contains("middle") ||
           value.contains("block") || value.contains("inline") || value.contains("flex") ||
           value.contains("grid") || value.contains("relative") || value.contains("absolute") ||
           value.contains("fixed") || value.contains("static") || value.contains("sticky") ||
           value.contains("hidden") || value.contains("visible") || value.contains("scroll") ||
           value.contains("solid") || value.contains("dashed") || value.contains("dotted") ||
           value.contains("thin") || value.contains("medium") || value.contains("thick") ||
           value.contains("serif") || value.contains("sans-serif") || value.contains("monospace") ||
           value.contains("cursive") || value.contains("fantasy") || value.contains("system-ui") ||
           value.contains("calc(") || value.contains("var(") || value.contains("linear-gradient") ||
           value.contains("radial-gradient") || value.contains("conic-gradient") ||
           value.contains("matrix(") || value.contains("translate") || value.contains("scale") ||
           value.contains("rotate") || value.contains("skew") || value.contains("perspective") ||
           value.contains("blur") || value.contains("brightness") || value.contains("contrast") ||
           value.contains("drop-shadow") || value.contains("grayscale") || value.contains("hue-rotate") ||
           value.contains("invert") || value.contains("opacity") || value.contains("saturate") ||
           value.contains("sepia") || value.contains("url(") {
            return true;
        }
        
        // 数値のみの場合も許可
        if value.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
            return true;
        }
        
        false
    }
    
    /// CSS文字列全体をサニタイズ
    pub fn sanitize_css_string(&self, css_string: &str) -> String {
        let mut sanitized_properties = Vec::new();
        
        // CSSプロパティを解析
        for property_declaration in css_string.split(';') {
            let property_declaration = property_declaration.trim();
            if property_declaration.is_empty() {
                continue;
            }
            
            if let Some(colon_pos) = property_declaration.find(':') {
                let property = &property_declaration[..colon_pos].trim();
                let value = &property_declaration[colon_pos + 1..].trim();
                
                if let Some(sanitized) = self.sanitize_css_property(property, value) {
                    sanitized_properties.push(sanitized);
                }
            }
        }
        
        sanitized_properties.join("; ")
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
        
        assert_eq!(
            sanitizer.sanitize_css_string("background-color: red; javascript: alert('xss');"),
            "background-color: red"
        );
        
        assert_eq!(
            sanitizer.sanitize_css_string("expression(alert('xss')); color: blue;"),
            "color: blue"
        );
    }
    
    #[test]
    fn test_sanitize_values() {
        let sanitizer = CssSanitizer::new();
        
        // 安全な値
        assert_eq!(
            sanitizer.sanitize_css_string("color: #ffffff; background-color: rgb(255, 0, 0);"),
            "color: #ffffff; background-color: rgb(255, 0, 0)"
        );
        
        // 危険な値
        assert_eq!(
            sanitizer.sanitize_css_string("background-image: url(javascript:alert('xss'));"),
            ""
        );
    }
}